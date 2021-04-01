extern crate regex;
extern crate termion;

use chrono::offset::{Local, Utc};
use figlet_rs::FIGfont;
use regex::Regex;
use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::async_stdin;
use termion::raw::IntoRawMode;

enum State {
    Utc,
    Local,
}

fn write_menu<W: Write>(stdout: &mut W, state: &State, x: u16, y: u16) {
    write!(
        stdout,
        "{}q: EXIT, u: UTC, l: Local, now: {}",
        termion::cursor::Goto(x, y),
        match state {
            State::Utc => "UTC",
            State::Local => "Local",
        }
    )
    .unwrap();
}

fn get_time_format(state: &State) -> String {
    match state {
        State::Utc => format!("{}", Utc::now().format("%r")),
        State::Local => format!("{}", Local::now().format("%r")),
    }
}

fn main() {
    let font = FIGfont::standand().unwrap();
    let re = Regex::new("\n").unwrap();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut state: State = State::Local;

    loop {
        match stdin.next() {
            Some(Ok(b'q')) => {
                break;
            }
            Some(Ok(b'u')) => state = State::Utc,
            Some(Ok(b'l')) => state = State::Local,
            _ => {}
        }
        write!(stdout, "{}", termion::clear::All).unwrap();
        write_menu(&mut stdout, &state, 1, 1);

        thread::sleep(Duration::from_millis(500));
        let time = get_time_format(&state);
        let figure = format!("{}", font.convert(&time).unwrap());
        let figure = re.replace_all(&figure, "\n\r");
        write!(stdout, "{}{}", termion::cursor::Goto(1, 2), figure).unwrap();

        stdout.flush().unwrap();
    }
}
