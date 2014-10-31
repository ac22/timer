timer.rs
A simple command line timer which takes a single argument, the time to wait in
minutes. After the given number of minutes passes, it will play alarm.wav on
loop until the user presses enter to turn off the alarm.

Compile:	rustc timer.rs
Run:		./timer 10			
(The above example would set a timer for 10 minutes.)

Tested on rustc 0.13.0
