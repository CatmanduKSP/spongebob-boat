use gilrs::{Axis, Button, Event, EventType, Gilrs};
use rppal::gpio::Gpio;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::time::Duration;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {

    //controller input crate init
    let mut gilrs = Gilrs::new().unwrap();

    //motor and low-shorting PWM setup


    //mutable variables
    let mut gear: i8 = 0;
    let mut speed: f32 = 0.0;
    let mut brake: f32 = 0.0;
    let mut brake_left: f32 = 0.0;
    let mut brake_right: f32 = 0.0;
    let mut turn_radius: f32 = 1.0;
    let mut turn: f32 = 0.0;
    let mut park: bool = false;
    let mut plug: bool = false;
    let mut left_speed: f32 = 0.0;
    let mut right_speed: f32 = 0.0;

    //pin init
    let mut left_forward = Pwm::with_frequency(
        Channel::Pwm2,
        500.0,
        0.0,
        Polarity::Normal,
        true
    )?;
    let mut left_back = Pwm::with_frequency(
        Channel::Pwm3,
        500.0,
        0.0,
        Polarity::Normal,
        true
    )?;
    let mut right_forward = Pwm::with_frequency(
        Channel::Pwm1,
        500.0,
        0.0,
        Polarity::Normal,
        true
    )?;
    let mut right_back = Pwm::with_frequency(
        Channel::Pwm0,
        500.0,
        0.0,
        Polarity::Normal,
        true
    )?;
    let mut left_brake = Gpio::new()?.get(22)?.into_output();
    let mut right_brake = Gpio::new()?.get(23)?.into_output();

    //main loop
    loop {

        //controller input fetch, switchboard
        while let Some(Event { event, .. }) = gilrs.next_event() {
            match event {
                //speed/brake
                EventType::AxisChanged(Axis::LeftStickY, value, _) => {
                    if value > 0.0 {
                        speed = value;
                        brake = 0.0;
                    } else if value < 0.0 {
                        brake = value.abs();
                        speed = 0.0;
                    } else {
                        speed = 0.0;
                        brake = 0.0;
                    }
                }
                //turn
                EventType::AxisChanged(Axis::LeftStickX, value, _) => {
                    turn = value;
                }
                //gear
                EventType::ButtonPressed(Button::RightTrigger2, _) => {
                    if gear < 1 {
                        gear += 1;
                    }
                }
                EventType::ButtonPressed(Button::LeftTrigger2, _) => {
                    if gear > -1 {
                        gear -= 1;
                    }
                }
                //park
                EventType::ButtonPressed(Button::RightThumb, _) => {
                    if park {
                        park = false;
                    } else {
                        park = true;
                    }
                }
                //brake plug
                EventType::ButtonPressed(Button::LeftThumb, _) => {
                    plug = true;
                }
                EventType::ButtonReleased(Button::LeftThumb, _) => {
                    plug = false;
                }
                //catch-all
                _ => {}
            }
        }
    
        //speed-dependent turning radius logic
        turn_radius = 1.0 - (speed * 0.7);
        //Motor Math
        if plug {
            left_speed = -1.0 * gear as f32;
            right_speed = -1.0 * gear as f32;
            brake = 0.0;
        } else if park {
            left_speed = turn;
            right_speed = -turn;
            brake = 0.0;
        } else {
            left_speed = ((speed + (turn * turn_radius * speed)) * gear as f32).clamp(-1.0, 1.0);
            right_speed = ((speed - (turn * turn_radius * speed)) * gear as f32).clamp(-1.0, 1.0);
            brake_left = (brake - (turn * turn_radius * brake)).clamp(0.0, 1.0);
            brake_right = (brake + (turn * turn_radius * brake)).clamp(0.0, 1.0);
        }

        //send speed to motors
        if left_speed < 0.0 { 
            left_forward.set_duty_cycle(0.0)?;
            left_back.set_duty_cycle(left_speed.abs() as f64)?;
        } else {
            left_forward.set_duty_cycle(left_speed.abs() as f64)?;
            left_back.set_duty_cycle(0.0)?;
        }
        if right_speed < 0.0 { 
            right_forward.set_duty_cycle(0.0)?;
            right_back.set_duty_cycle(right_speed.abs() as f64)?;
        } else {
            right_forward.set_duty_cycle(right_speed.abs() as f64)?;
            right_back.set_duty_cycle(0.0)?;
        }

        //send brake% to motors
        left_brake.set_pwm_frequency(500.0, brake_left as f64)?;
        right_brake.set_pwm_frequency(500.0, brake_right as f64)?;

        //closing
        std::thread::sleep(Duration::from_millis(20));
    }
}
