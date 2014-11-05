// timer.rs - command line timer that plays alarm.wav looped after elapsed time
// Copyright (C) 2014 Anand Chandran
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://gnu.org/licenses/>.
//

extern crate getopts;
use getopts::{optflag, getopts, OptGroup};
use std::os;
use std::io;
use std::time::Duration;

// plays the sound file alarm.wav until the user presses enter
fn play_sound() {
    let sound_file = "alarm.wav";
    println!("Timer done - press enter!");
    let mut process = io::Command::new("cvlc")
        .args([sound_file, "--loop"])
        .spawn().ok().expect("Failed to execute process.");

    for _line in io::stdin().lines() {
        process.signal_kill();
        return;
    }
}

// waits n seconds
fn wait_n_seconds(n: uint) {
    let mut timer = io::Timer::new().unwrap();
    let periodic = timer.periodic(Duration::seconds(1));

    for _i in range(1, n + 1) {
        periodic.recv();
    }
}

// prints how to use the program
fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [-h] minutes", program);
    println!("-h --help\tUsage");
}

// parse the arguments, if they are malformed display the help
// otherwise wait the given number of minutes and then loop playing
// alarm.wav until the user presses enter
fn main() {
    let args = os::args();
    let program = args[0].clone();

    let opts = [
        optflag("h", "help", "print this help menu")
        ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m)   => m,
        Err(f)  => { print_usage(program.as_slice(), opts); 
            panic!(f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(program.as_slice(), opts);
        return;
    };

    let mins: Option<uint> = from_str(input.as_slice().trim());

    match mins {
        Some(number)    => {
            wait_n_seconds(60 * number);
            play_sound();
        },
        None            =>  { 
            print_usage(program.as_slice(), opts);
        }
    }
}
