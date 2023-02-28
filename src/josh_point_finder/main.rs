use core::time;
use std::{thread, time::Instant};

use autopilot::{geometry::Point, mouse::{Button, self}};

fn main() {
    let now = Instant::now();
    loop {
        println!("{}", autopilot::mouse::location());
        if now.elapsed().as_secs() > 10{
            break;
        }
    }
}