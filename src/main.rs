extern crate termion;

mod core;
mod unicode;

use crate::core::direction::Direction;
use crate::core::level::Level;
use crate::unicode::*;

use std::convert::TryInto;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn print_level(level: &Level, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    write!(stdout, "{}", termion::clear::All).unwrap();

    let lines = get_level_lines(level);
    let mut line_number = 0;
    for line in lines {
        line_number += 1;
        write!(stdout, "{}{}", termion::cursor::Goto(1, line_number), line).unwrap();
    }
    line_number += 1;
    write!(
        stdout,
        "{}Rules :",
        termion::cursor::Goto(1, line_number.try_into().unwrap())
    )
    .unwrap();

    for rule in &level.rules {
        line_number += 1;
        write!(
            stdout,
            "{}  - {:?} is {:?}",
            termion::cursor::Goto(1, line_number.try_into().unwrap()),
            rule.noun,
            rule.adjective
        )
        .unwrap();
    }
    stdout.flush().unwrap();
}

fn main() {
    let mut level = build_level_from_file(String::from("levels/test.txt"));

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    print_level(&level, &mut stdout);

    for c in stdin.keys() {
        let mut is_win = false;
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Esc => break,
            Key::Char('u') => level.undo(),
            Key::Backspace => level.undo(),
            Key::Left => is_win = level.next(Direction::LEFT),
            Key::Right => is_win = level.next(Direction::RIGHT),
            Key::Up => is_win = level.next(Direction::UP),
            Key::Down => is_win = level.next(Direction::DOWN),
            _ => {}
        }
        print_level(&level, &mut stdout);

        if is_win {
            break;
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
