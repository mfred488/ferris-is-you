extern crate termion;

mod core;
mod unicode;

use crate::core::direction::Direction;
use crate::core::level::Level;
use crate::core::rule::Rule;
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
        match rule {
            Rule::NounIsNominalRule(noun_is_nominal_rule) => {
                write!(
                    stdout,
                    "{}  - {:?} is {:?}",
                    termion::cursor::Goto(1, line_number.try_into().unwrap()),
                    noun_is_nominal_rule.noun,
                    noun_is_nominal_rule.nominal
                )
                .unwrap();
            }
            Rule::NounsGroupIsNominalsGroupRule(nouns_is_nominals_rule) => {
                write!(
                    stdout,
                    "{}  - {:?} is {:?}",
                    termion::cursor::Goto(1, line_number.try_into().unwrap()),
                    nouns_is_nominals_rule.nouns,
                    nouns_is_nominals_rule.nominals,
                )
                .unwrap();
            }
        }
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
            Key::Char(' ') => is_win = level.next(None),
            Key::Left => is_win = level.next(Some(Direction::LEFT)),
            Key::Right => is_win = level.next(Some(Direction::RIGHT)),
            Key::Up => is_win = level.next(Some(Direction::UP)),
            Key::Down => is_win = level.next(Some(Direction::DOWN)),
            _ => {}
        }
        print_level(&level, &mut stdout);

        if is_win {
            break;
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
