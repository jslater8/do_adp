use core::time;
use std::thread;

use autopilot::{geometry::Point, mouse::{Button, self}};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
   right_arrow_x: f64,
   right_arrow_y: f64,
   download_x: f64,
   download_y: f64,
   num_files: u64,
   #[arg(default_value_t = 5)]
   time_between_clicks: u64,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.num_files-1{
        mouse::move_to(Point{x: args.download_x, y:args.download_y}).unwrap();
        mouse::click(Button::Left, None);
        thread::sleep(time::Duration::from_secs(args.time_between_clicks));

        mouse::move_to(Point{x: args.right_arrow_x, y:args.right_arrow_y}).unwrap();
        mouse::click(Button::Left, None);
        thread::sleep(time::Duration::from_secs(args.time_between_clicks));
    }
    mouse::move_to(Point{x: args.download_x, y:args.download_y}).unwrap();
    mouse::click(Button::Left, None);
    thread::sleep(time::Duration::from_secs(args.time_between_clicks));
}