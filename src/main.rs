extern crate regex;
extern crate termion;

use chrono::Local;
use figlet_rs::FIGfont;
use regex::Regex;
use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::async_stdin;
use termion::raw::IntoRawMode;

fn main() {
    let font = FIGfont::standand().unwrap();
    let re = Regex::new("\n").unwrap();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }
        write!(stdout, "{}", termion::clear::All);
        write!(stdout, "{}q: exit", termion::cursor::Goto(1, 1));

        thread::sleep(Duration::from_millis(1000));
        let time = format!("{}", Local::now().format("%r"));
        let figure = format!("{}", font.convert(&time).unwrap());
        let figure = re.replace_all(&figure, "\n\r");
        write!(stdout, "{}{}", termion::cursor::Goto(1, 2), figure);

        stdout.flush().unwrap();
    }
}
