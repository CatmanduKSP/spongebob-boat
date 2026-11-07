# spongebob-boat-drive

## Code for a Raspberry Pi that controls a 2-seater go-kart modeled after Spongebob's boat
A Rust program that allows a Raspberry Pi to take input from a UART steering wheel or other USB gamepad and output PWM to two IBT_2 H-bridge motor driver modules, which in the original project, power two salvaged wheelchair motors that drive a lightweight 2-person go-kart decorated to look like Spongebob's boat for the 2026 Hebron High School homecoming parade.
The code features a gear system with reverse, neutral, and forward; a "clutch-kick" functionality where holding the accelerator in neutral and then shifting to either forward or reverse causes the motors to push full power; and a "tank turn" mode where pedals are disabled and the steering wheel rotates the kart about the center of the rear axle.

### Design choices
The code runs on a 20ms control loop to ensure imperceptible input delay. Vehicle speed is estimated for braking and acceleration purposes so encoders are not necessary. Neutral gear disengages the motors to make the boat easier to tow, and the boat "remembers" speed in much the same way a real car's drivetrain would, so if the wheels are spinning in forwards and the boat is suddenly put into reverse, the motors plug and push power backwards immediately. The PWM runs at 500hz to improve low-end "kick" at the cost of perceivable noise level.

### Input guide
Left stick y-axis control acceleration and brake
Left stick x-axis controls turning
Left trigger decreases gear and right trigger increases it
Left thumb button sets both motors to full backwards power immediately; ideal for emergency brake
Right thumb button toggles park state, which lets the kart tank-turn
