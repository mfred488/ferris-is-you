extern crate termion;

use std::convert::TryInto;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Object {
    FERRIS,
    ROCKET,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Word {
    FERRIS,
    ROCKET,
    IS,
    YOU,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Element {
    Object(Object),
    Word(Word),
}

enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

fn get_printable_character(element: &Option<Element>) -> String {
    match element {
        Some(Element::Object(Object::FERRIS)) => return String::from("🦀"),
        Some(Element::Object(Object::ROCKET)) => return String::from("🚀"),
        Some(Element::Word(Word::FERRIS)) => return String::from("Fe"),
        Some(Element::Word(Word::ROCKET)) => return String::from("Ro"),
        Some(Element::Word(Word::IS)) => return String::from("=="),
        Some(Element::Word(Word::YOU)) => return String::from("U "),
        None => return String::from(".."),
        // _ => return String::from("?"),
    };
}

struct ElementWithLocation {
    x: usize,
    y: usize,
    element: Element,
}

struct Level {
    width: usize,
    height: usize,
    grid: Vec<Option<Element>>,
    elements_with_locations: Vec<ElementWithLocation>,
}

fn get_word(object: &Object) -> Word {
    match object {
        Object::FERRIS => Word::FERRIS,
        Object::ROCKET => Word::ROCKET,
    }
}

impl Level {
    fn new(width: usize, height: usize) -> Level {
        assert!(width > 3 && height > 3); // Required for is_you
        let mut level = Level {
            width,
            height,
            grid: Vec::with_capacity(width * height),
            elements_with_locations: Vec::new(),
        };

        for _ in 0..height * width {
            level.grid.push(None)
        }
        level
    }

    fn get_grid_index(&self, x: usize, y: usize) -> usize {
        x * self.width + y
    }

    fn clear_grid(&mut self) {
        self.grid.clear();
        for _ in 0..self.height * self.width {
            self.grid.push(None)
        }
    }

    fn add_object(&mut self, x: usize, y: usize, element: Element) {
        let index = self.get_grid_index(x, y);
        self.grid[index] = Some(element);
        self.elements_with_locations
            .push(ElementWithLocation { x, y, element })
    }

    fn next(&mut self, direction: Direction) {
        let mut new_elements_with_locations: Vec<ElementWithLocation> =
            Vec::with_capacity(self.elements_with_locations.len());

        for element_with_location in &self.elements_with_locations {
            let mut new_x = element_with_location.x;
            let mut new_y = element_with_location.y;
            if self.is_you(&element_with_location.element) {
                match direction {
                    Direction::UP => new_y = if new_y == 0 { 0 } else { new_y - 1 },
                    Direction::DOWN => {
                        new_y = if new_y == self.height - 1 {
                            self.height - 1
                        } else {
                            new_y + 1
                        }
                    }
                    Direction::RIGHT => {
                        new_x = if new_x == self.width - 1 {
                            self.width - 1
                        } else {
                            new_x + 1
                        }
                    }
                    Direction::LEFT => new_x = if new_x == 0 { 0 } else { new_x - 1 },
                }
            }

            new_elements_with_locations.push(ElementWithLocation {
                x: new_x,
                y: new_y,
                element: element_with_location.element,
            });
        }

        self.elements_with_locations = new_elements_with_locations;
        self.clear_grid();
        for element_with_location in &self.elements_with_locations {
            let index = self.get_grid_index(element_with_location.x, element_with_location.y);
            self.grid[index] = Some(element_with_location.element);
        }
    }

    fn is_you(&self, element: &Element) -> bool {
        match element {
            Element::Word(_) => false,
            Element::Object(object) => {
                // Vertical rules
                for x in 0..self.width {
                    for y in 0..self.height - 2 {
                        match (
                            self.grid[self.get_grid_index(x, y)],
                            self.grid[self.get_grid_index(x, y + 1)],
                            self.grid[self.get_grid_index(x, y + 2)],
                        ) {
                            (
                                Some(Element::Word(word)),
                                Some(Element::Word(Word::IS)),
                                Some(Element::Word(Word::YOU)),
                            ) => {
                                if word == get_word(&object) {
                                    return true;
                                } else {
                                    continue;
                                }
                            }
                            _ => continue,
                        }
                    }
                }

                // Horizontal rules
                for y in 0..self.height {
                    for x in 0..self.width - 2 {
                        match (
                            self.grid[self.get_grid_index(x, y)],
                            self.grid[self.get_grid_index(x + 1, y)],
                            self.grid[self.get_grid_index(x + 2, y)],
                        ) {
                            (
                                Some(Element::Word(word)),
                                Some(Element::Word(Word::IS)),
                                Some(Element::Word(Word::YOU)),
                            ) => {
                                if word == get_word(&object) {
                                    return true;
                                } else {
                                    continue;
                                }
                            }
                            _ => continue,
                        }
                    }
                }

                false
            }
        }
    }

    fn print(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        write!(stdout, "{}", termion::clear::All).unwrap();
        for y in 0..self.height {
            let mut line = String::with_capacity(self.width);
            for x in 0..self.width {
                line.push_str(&get_printable_character(
                    &self.grid[self.get_grid_index(x, y)],
                ))
            }
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, y.try_into().unwrap()),
                line
            )
            .unwrap();
        }
        stdout.flush().unwrap();
    }
}

fn main() {
    let mut level = Level::new(12, 12);
    level.add_object(2, 3, Element::Object(Object::FERRIS));
    level.add_object(6, 6, Element::Object(Object::FERRIS));
    level.add_object(0, 5, Element::Object(Object::ROCKET));
    level.add_object(7, 9, Element::Word(Word::FERRIS));
    level.add_object(8, 9, Element::Word(Word::IS));
    level.add_object(9, 9, Element::Word(Word::YOU));

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
    level.print(&mut stdout);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Esc => break,
            Key::Left => level.next(Direction::LEFT),
            Key::Right => level.next(Direction::RIGHT),
            Key::Up => level.next(Direction::UP),
            Key::Down => level.next(Direction::DOWN),
            _ => {}
        }
        level.print(&mut stdout);
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
