extern crate termion;

use chrono::Local;
use std::thread;
use std::time::Duration;
use figlet_rs::FIGfont;

fn main() {
    let font = FIGfont::standand().unwrap();
    loop {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
        thread::sleep(Duration::from_millis(1000));
        let time = format!("{}", Local::now().format("%r"));
        let figure = format!("{}", font.convert(&time).unwrap());
        print!("{}", figure);
    }
}