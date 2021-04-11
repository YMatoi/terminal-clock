extern crate regex;
extern crate termion;

use chrono::offset::{Local, Utc};
use figlet_rs::FIGfont;
use regex::Regex;
use std::io::{stdout, Read, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use termion::async_stdin;
use termion::raw::IntoRawMode;

enum State {
    Utc,
    Local,
}

enum Event {
    UpdateTime,
    UpdateTimeState(State),
    Quit,
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
    let regex = Regex::new("\n").unwrap();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let (ts, rs): (Sender<Event>, Receiver<Event>) = mpsc::channel();
    let (te, re): (Sender<bool>, Receiver<bool>) = mpsc::channel();

    let key_event = std::thread::spawn(move || loop {
        match stdin.next() {
            Some(Ok(b'q')) => {
                ts.send(Event::Quit).unwrap();
                te.send(true).unwrap();
                break;
            }
            Some(Ok(b'u')) => ts.send(Event::UpdateTimeState(State::Utc)).unwrap(),
            Some(Ok(b'l')) => ts.send(Event::UpdateTimeState(State::Local)).unwrap(),
            _ => {}
        }
    });

    let (tt, rt): (Sender<Event>, Receiver<Event>) = mpsc::channel();

    let time_event = std::thread::spawn(move || loop {
        if let Ok(true) = re.recv_timeout(Duration::from_millis(500)) {
            break;
        };
        tt.send(Event::UpdateTime).unwrap();
    });

    let mut state: State = State::Local;

    loop {
        let mut update = rt.recv_timeout(Duration::from_millis(0)).is_ok();
        if let Ok(event) = rs.recv_timeout(Duration::from_millis(0)) {
            match event {
                Event::UpdateTimeState(s) => {
                    state = s;
                    update = true;
                }
                Event::Quit => {
                    break;
                }
                _ => {}
            }
        }

        if update {
            write!(stdout, "{}", termion::clear::All).unwrap();
            write_menu(&mut stdout, &state, 1, 1);

            let time = get_time_format(&state);
            let figure = format!("{}", font.convert(&time).unwrap());
            let figure = regex.replace_all(&figure, "\n\r");
            write!(stdout, "{}{}", termion::cursor::Goto(1, 2), figure).unwrap();

            stdout.flush().unwrap();
        }
    }

    key_event.join().unwrap();
    time_event.join().unwrap();
}
