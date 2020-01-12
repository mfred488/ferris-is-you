extern crate termion;

mod core;

use crate::core::direction::Direction;
use crate::core::element::*;
use crate::core::level::Level;

use std::convert::TryInto;
use std::fs;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use unicode_segmentation::UnicodeSegmentation;

fn get_printable_character(element: Option<&Element>) -> &str {
    match element {
        Some(Element::Object(Object::FERRIS)) => return "🦀",
        Some(Element::Object(Object::ROCKET)) => return "🚀",
        Some(Element::Object(Object::FLAG)) => return "🚩",
        Some(Element::Object(Object::WALL)) => return "🧱",
        Some(Element::Object(Object::WATER)) => return "🌊",
        Some(Element::Text(Text::Noun(Noun::FERRIS))) => return "Fe",
        Some(Element::Text(Text::Noun(Noun::ROCKET))) => return "Ro",
        Some(Element::Text(Text::Noun(Noun::FLAG))) => return "Fg",
        Some(Element::Text(Text::Noun(Noun::WALL))) => return "Wa",
        Some(Element::Text(Text::Noun(Noun::WATER))) => return "Wt",
        Some(Element::Text(Text::Noun(Noun::TEXT))) => return "Te",
        Some(Element::Text(Text::IS)) => return "==",
        Some(Element::Text(Text::Adjective(Adjective::YOU))) => return "U ",
        Some(Element::Text(Text::Adjective(Adjective::WIN))) => return "Wi",
        Some(Element::Text(Text::Adjective(Adjective::STOP))) => return "St",
        Some(Element::Text(Text::Adjective(Adjective::PUSH))) => return "Pu",
        Some(Element::Text(Text::Adjective(Adjective::SINK))) => return "Si",
        Some(Element::Text(Text::Adjective(Adjective::FLOAT))) => return "Fl",
        None => return "..",
        // _ => return String::from("?"),
    };
}

// Kind of reverse function of the function above; try to de-duplicate that, but without dropping the match (that will detect conflicting definitions)
fn get_element_from_printable_character(chars: &str) -> Option<Element> {
    match chars {
        "🦀" => Some(Element::Object(Object::FERRIS)),
        "🚀" => Some(Element::Object(Object::ROCKET)),
        "🚩" => Some(Element::Object(Object::FLAG)),
        "🧱" => Some(Element::Object(Object::WALL)),
        "🌊" => Some(Element::Object(Object::WATER)),
        "Fe" => Some(Element::Text(Text::Noun(Noun::FERRIS))),
        "Ro" => Some(Element::Text(Text::Noun(Noun::ROCKET))),
        "Fg" => Some(Element::Text(Text::Noun(Noun::FLAG))),
        "Wa" => Some(Element::Text(Text::Noun(Noun::WALL))),
        "Wt" => Some(Element::Text(Text::Noun(Noun::WATER))),
        "Te" => Some(Element::Text(Text::Noun(Noun::TEXT))),
        "==" => Some(Element::Text(Text::IS)),
        "U " => Some(Element::Text(Text::Adjective(Adjective::YOU))),
        "Wi" => Some(Element::Text(Text::Adjective(Adjective::WIN))),
        "St" => Some(Element::Text(Text::Adjective(Adjective::STOP))),
        "Pu" => Some(Element::Text(Text::Adjective(Adjective::PUSH))),
        "Si" => Some(Element::Text(Text::Adjective(Adjective::SINK))),
        "Fl" => Some(Element::Text(Text::Adjective(Adjective::FLOAT))),
        ".." => None,
        _ => panic!("Unknown character {}", chars),
    }
}

fn print_level(level: &Level, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    write!(stdout, "{}", termion::clear::All).unwrap();
    let mut line_number = 0;
    for y in 0..level.height {
        line_number += 1;
        let mut line = String::with_capacity(level.width);
        for x in 0..level.width {
            let first_element = level.get_elements(x, y).get(0);
            line.push_str(get_printable_character(first_element))
        }
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

fn build_level_from_file(file_path: String) -> Level {
    let file_content = fs::read_to_string(file_path).expect("Could not open file !");
    let lines = file_content.lines();

    let mut width = 0;
    let mut height = 0;
    let mut elements_to_add: Vec<(usize, usize, Element)> = Vec::new();

    for line in lines {
        if line.starts_with("#") {
            continue;
        }

        let mut local_width = 0;
        let graphemes = UnicodeSegmentation::graphemes(line, true).collect::<Vec<&str>>();
        let mut previous_grapheme = "";

        for grapheme in graphemes {
            if grapheme.is_ascii() {
                local_width += 1;
                if (local_width % 2) == 0 {
                    if let Some(element) = get_element_from_printable_character(
                        &(previous_grapheme.to_owned() + grapheme),
                    ) {
                        elements_to_add.push((local_width / 2 - 1, height, element))
                    }
                } else {
                    previous_grapheme = grapheme;
                }
            } else {
                local_width += 2;
                if let Some(element) = get_element_from_printable_character(grapheme) {
                    elements_to_add.push((local_width / 2 - 1, height, element));
                }
            }
        }

        if width > 0 && local_width != width * 2 {
            panic!(
                "The width of the line {} is inconsistent with the one of previous line(s)!",
                &line
            );
        }
        height += 1;
        width = local_width / 2;
    }

    let mut level = Level::new(width, height);
    for (x, y, element) in elements_to_add {
        level.add_element(x, y, element);
    }

    level
}
