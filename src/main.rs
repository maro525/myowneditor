extern crate clap;
extern crate termion;

use clap::{App, Arg};
use std::cmp::min;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cursor {
    row: usize,
    column: usize
}

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

    // cursor
    let mut cursor = Cursor { row: 0, column: 0 };

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
        match evt.unwrap() {
            Event::Key(Key::Ctrl('c')) => {
                return;
            }
            
            Event::Key(Key::Up) => {
                if cursor.row > 0 {
                    cursor.row -= 1;
                    cursor.column = min(buffer[cursor.row].len(), cursor.column);
                }
            }
            Event::Key(Key::Down) => {
                if cursor.row + 1 < buffer.len() {
                    cursor.row += 1;
                    cursor.column = min(cursor.column, buffer[cursor.row].len());
                }
            }
            Event::Key(Key::Left) =>  {
                if cursor.column > 0 {
                    cursor.column -= 1;
                }
            }
            Event::Key(Key::Right) => {
                cursor.column = min(cursor.column + 1, buffer[cursor.row].len());
            }
            _ => {}
        }

        write!(
            stdout,
            "{}",
            cursor::Goto(cursor.column as u16 + 1, cursor.row as u16 + 1)
        );

        stdout.flush().unwrap();
    }
}
