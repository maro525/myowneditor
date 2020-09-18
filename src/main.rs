extern crate clap;
extern crate termion;

use clap::{App, Arg};
use std::ffi::OsStr;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path;
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

fn main() {
    // clap
    let matches = App::new("myowneditor")
        .about("A text editor")
        .bin_name("myowneditor")
        .arg(Arg::with_name("file"))
        .get_matches();

    // file
    let file_path: Option<&OsStr> = matches.value_of_os("file");

    // read text
    let buffer: Vec<Vec<char>> = file_path
        .and_then(|file_path| {
            fs::read_to_string(path::Path::new(file_path))
                .ok()
                .map(|s| {
                    s.lines()
                        .map(|line| line.trim_right().chars().collect())
                        .collect()
                })
        })
        .unwrap_or(Vec::new());

    // std
    let stdin = stdin();

    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // clear screen
    write!(stdout, "{}", clear::All);
    // cursor
    write!(stdout, "{}", cursor::Goto(1, 1));

    // test pring
    // write!(stdout, "{}", "Hello World!");
    
    for line in &buffer {
        for &c in line {
            write!(stdout, "{}", c);
        }

        write!(stdout, "\r\n");
    }

    // flash
    stdout.flush().unwrap();

    // events
    for evt in stdin.events() {
        if evt.unwrap() == Event::Key(Key::Ctrl('c')) {
            return;
        }
    }
}
