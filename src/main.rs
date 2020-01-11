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
    FLAG,
    WALL,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Word {
    Noun(Noun),
    IS,
    Adjective(Adjective),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Noun {
    FERRIS,
    ROCKET,
    FLAG,
    WALL,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Adjective {
    YOU,
    WIN,
    STOP,
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

fn get_printable_character(element: Option<&Element>) -> String {
    match element {
        Some(Element::Object(Object::FERRIS)) => return String::from("🦀"),
        Some(Element::Object(Object::ROCKET)) => return String::from("🚀"),
        Some(Element::Object(Object::FLAG)) => return String::from("🚩"),
        Some(Element::Object(Object::WALL)) => return String::from("🧱"),
        Some(Element::Word(Word::Noun(Noun::FERRIS))) => return String::from("Fe"),
        Some(Element::Word(Word::Noun(Noun::ROCKET))) => return String::from("Ro"),
        Some(Element::Word(Word::Noun(Noun::FLAG))) => return String::from("Fl"),
        Some(Element::Word(Word::Noun(Noun::WALL))) => return String::from("Wa"),
        Some(Element::Word(Word::IS)) => return String::from("=="),
        Some(Element::Word(Word::Adjective(Adjective::YOU))) => return String::from("U "),
        Some(Element::Word(Word::Adjective(Adjective::WIN))) => return String::from("Wi"),
        Some(Element::Word(Word::Adjective(Adjective::STOP))) => return String::from("St"),
        None => return String::from(".."),
        // _ => return String::from("?"),
    };
}

#[derive(Debug)]
struct ElementWithLocation {
    x: usize,
    y: usize,
    element: Element,
}

struct Level {
    width: usize,
    height: usize,
    grid: Vec<Vec<Element>>,
    elements_with_locations: Vec<ElementWithLocation>,
}

fn get_noun(object: &Object) -> Noun {
    match object {
        Object::FERRIS => Noun::FERRIS,
        Object::ROCKET => Noun::ROCKET,
        Object::FLAG => Noun::FLAG,
        Object::WALL => Noun::WALL,
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
            level.grid.push(Vec::new())
        }
        level
    }

    fn get_grid_index(&self, x: usize, y: usize) -> usize {
        x * self.width + y
    }

    fn clear_grid(&mut self) {
        self.grid.clear();
        for _ in 0..self.height * self.width {
            self.grid.push(Vec::new())
        }
    }

    fn add_object(&mut self, x: usize, y: usize, element: Element) {
        let index = self.get_grid_index(x, y);
        self.grid[index].push(element);
        self.elements_with_locations
            .push(ElementWithLocation { x, y, element })
    }

    fn next(&mut self, direction: Direction) -> bool {
        let mut new_elements_with_locations: Vec<ElementWithLocation> =
            Vec::with_capacity(self.elements_with_locations.len());

        for element_with_location in &self.elements_with_locations {
            let mut new_x = element_with_location.x;
            let mut new_y = element_with_location.y;
            if self.is_adjective(&element_with_location.element, Adjective::YOU) {
                match self.can_move(&element_with_location, &direction) {
                    Some((local_new_x, local_new_y)) => {
                        new_x = local_new_x;
                        new_y = local_new_y;
                    }
                    None => {}
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
            self.grid[index].push(element_with_location.element);
        }

        self.is_win()
    }

    fn can_move(
        &self,
        element_with_location: &ElementWithLocation,
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        let mut new_x = element_with_location.x;
        let mut new_y = element_with_location.y;
        // Objects can't go off limits
        match direction {
            Direction::UP => {
                if new_y > 0 {
                    new_y = new_y - 1
                } else {
                    return None;
                }
            }
            Direction::DOWN => {
                if new_y < self.height - 1 {
                    new_y = new_y + 1
                } else {
                    return None;
                }
            }
            Direction::LEFT => {
                if new_x > 0 {
                    new_x = new_x - 1
                } else {
                    return None;
                }
            }
            Direction::RIGHT => {
                if new_x < self.width - 1 {
                    new_x = new_x + 1
                } else {
                    return None;
                }
            }
        }

        // Can't pass stop objects
        for element_in_next_location in &self.grid[self.get_grid_index(new_x, new_y)] {
            if self.is_adjective(&element_in_next_location, Adjective::STOP) {
                return None;
            }
        }

        Some((new_x, new_y))
    }

    fn is_adjective(&self, element: &Element, adjective: Adjective) -> bool {
        match element {
            Element::Word(_) => false,
            Element::Object(object) => {
                // Vertical rules
                for x in 0..self.width {
                    for y in 0..self.height - 2 {
                        for el1 in &self.grid[self.get_grid_index(x, y)] {
                            for el2 in &self.grid[self.get_grid_index(x, y + 1)] {
                                for el3 in &self.grid[self.get_grid_index(x, y + 2)] {
                                    match (el1, el2, el3) {
                                        (
                                            Element::Word(Word::Noun(noun)),
                                            Element::Word(Word::IS),
                                            Element::Word(Word::Adjective(local_adjective)),
                                        ) => {
                                            if noun == &get_noun(&object)
                                                && &adjective == local_adjective
                                            {
                                                eprintln!(
                                                    "{:?} is {:?} (vertical rule at {},{})",
                                                    object, adjective, x, y
                                                );
                                                return true;
                                            } else {
                                                continue;
                                            }
                                        }
                                        _ => continue,
                                    }
                                }
                            }
                        }
                    }
                }

                // Horizontal rules
                for y in 0..self.height {
                    for x in 0..self.width - 2 {
                        for el1 in &self.grid[self.get_grid_index(x, y)] {
                            for el2 in &self.grid[self.get_grid_index(x + 1, y)] {
                                for el3 in &self.grid[self.get_grid_index(x + 2, y)] {
                                    match (el1, el2, el3) {
                                        (
                                            Element::Word(Word::Noun(noun)),
                                            Element::Word(Word::IS),
                                            Element::Word(Word::Adjective(local_adjective)),
                                        ) => {
                                            if noun == &get_noun(&object)
                                                && &adjective == local_adjective
                                            {
                                                return true;
                                            } else {
                                                continue;
                                            }
                                        }
                                        _ => continue,
                                    }
                                }
                            }
                        }
                    }
                }

                false
            }
        }
    }

    fn is_win(&self) -> bool {
        for element_with_location in &self.elements_with_locations {
            if !self.is_adjective(&element_with_location.element, Adjective::YOU) {
                continue;
            }

            let elements_at_same_location =
                &self.grid[self.get_grid_index(element_with_location.x, element_with_location.y)];
            for element_at_same_location in elements_at_same_location {
                if self.is_adjective(element_at_same_location, Adjective::WIN) {
                    return true;
                }
            }
        }

        false
    }

    fn print(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        // write!(stdout, "{}", termion::clear::All).unwrap();
        for y in 0..self.height {
            let mut line = String::with_capacity(self.width);
            for x in 0..self.width {
                let first_element = self.grid[self.get_grid_index(x, y)].get(0);
                line.push_str(&get_printable_character(first_element))
            }
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, (1 + y).try_into().unwrap()),
                line
            )
            .unwrap();
        }
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(1, (1 + self.height).try_into().unwrap())
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}

fn main() {
    let mut level = Level::new(12, 12);
    level.add_object(2, 3, Element::Object(Object::FERRIS));
    level.add_object(1, 9, Element::Object(Object::FLAG));
    level.add_object(0, 0, Element::Object(Object::WALL));
    level.add_object(6, 6, Element::Object(Object::FERRIS));
    level.add_object(0, 5, Element::Object(Object::ROCKET));
    level.add_object(7, 9, Element::Word(Word::Noun(Noun::FERRIS)));
    level.add_object(8, 9, Element::Word(Word::IS));
    level.add_object(9, 9, Element::Word(Word::Adjective(Adjective::YOU)));
    level.add_object(7, 10, Element::Word(Word::Noun(Noun::ROCKET)));
    level.add_object(8, 10, Element::Word(Word::IS));
    level.add_object(9, 10, Element::Word(Word::Adjective(Adjective::WIN)));
    level.add_object(7, 11, Element::Word(Word::Noun(Noun::WALL)));
    level.add_object(8, 11, Element::Word(Word::IS));
    level.add_object(9, 11, Element::Word(Word::Adjective(Adjective::STOP)));

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
        let mut is_win = false;
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Esc => break,
            Key::Left => is_win = level.next(Direction::LEFT),
            Key::Right => is_win = level.next(Direction::RIGHT),
            Key::Up => is_win = level.next(Direction::UP),
            Key::Down => is_win = level.next(Direction::DOWN),
            _ => {}
        }
        level.print(&mut stdout);

        if is_win {
            break;
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
