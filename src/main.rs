extern crate termion;
use std::collections::VecDeque;
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
    TEXT,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Adjective {
    YOU,
    WIN,
    STOP,
    PUSH,
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

#[derive(Debug)]
struct NounIsAdjectiveRule {
    noun: Noun,
    adjective: Adjective,
}

fn get_rule(el1: &Element, el2: &Element, el3: &Element) -> Option<NounIsAdjectiveRule> {
    match (el1, el2, el3) {
        (
            Element::Word(Word::Noun(noun)),
            Element::Word(Word::IS),
            Element::Word(Word::Adjective(adjective)),
        ) => Some(NounIsAdjectiveRule {
            noun: noun.clone(),
            adjective: adjective.clone(),
        }),
        _ => None,
    }
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
        Some(Element::Word(Word::Noun(Noun::TEXT))) => return String::from("Te"),
        Some(Element::Word(Word::IS)) => return String::from("=="),
        Some(Element::Word(Word::Adjective(Adjective::YOU))) => return String::from("U "),
        Some(Element::Word(Word::Adjective(Adjective::WIN))) => return String::from("Wi"),
        Some(Element::Word(Word::Adjective(Adjective::STOP))) => return String::from("St"),
        Some(Element::Word(Word::Adjective(Adjective::PUSH))) => return String::from("Pu"),
        None => return String::from(".."),
        // _ => return String::from("?"),
    };
}

struct Level {
    width: usize,
    height: usize,
    grid: Vec<Vec<Element>>,
    old_grids: VecDeque<Vec<Vec<Element>>>,
    rules: Vec<NounIsAdjectiveRule>,
}

fn get_noun(element: &Element) -> Noun {
    match element {
        Element::Word(_) => Noun::TEXT,
        Element::Object(Object::FERRIS) => Noun::FERRIS,
        Element::Object(Object::ROCKET) => Noun::ROCKET,
        Element::Object(Object::FLAG) => Noun::FLAG,
        Element::Object(Object::WALL) => Noun::WALL,
    }
}

impl Level {
    fn new(width: usize, height: usize) -> Level {
        assert!(width > 3 && height > 3); // Required for is_you
        let mut level = Level {
            width,
            height,
            grid: Vec::with_capacity(width * height),
            old_grids: VecDeque::new(),
            rules: Vec::new(),
        };

        for _ in 0..height * width {
            level.grid.push(Vec::new())
        }
        level
    }

    fn get_grid_index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);
        x * self.height + y
    }

    fn add_object(&mut self, x: usize, y: usize, element: Element) {
        let index = self.get_grid_index(x, y);
        self.grid[index].push(element);
        self.build_rules();
    }

    fn undo(&mut self) {
        match self.old_grids.pop_back() {
            Some(grid) => self.grid = grid,
            None => {}
        }
    }

    fn next(&mut self, direction: Direction) -> bool {
        let mut new_grid: Vec<Vec<Element>> = self.grid.clone();
        let mut moves_to_do: VecDeque<(Element, usize, usize, &Direction)> = VecDeque::new();

        for x in 0..self.width {
            for y in 0..self.height {
                for element in &self.grid[self.get_grid_index(x, y)] {
                    if self.is_adjective(&element, Adjective::YOU) {
                        if self.can_move(x, y, &direction) {
                            moves_to_do.push_back((element.clone(), x, y, &direction));
                        }
                    }
                }
            }
        }

        while moves_to_do.len() > 0 {
            let (element_to_move, x, y, direction_to_move) = moves_to_do.pop_front().unwrap();
            let index_to_remove = new_grid[self.get_grid_index(x, y)]
                .iter()
                .position(|&el| el == element_to_move)
                .unwrap();
            new_grid[self.get_grid_index(x, y)].remove(index_to_remove);

            let mut new_x = x;
            let mut new_y = y;
            match direction {
                Direction::UP => new_y = new_y - 1,
                Direction::DOWN => new_y = new_y + 1,
                Direction::LEFT => new_x = new_x - 1,
                Direction::RIGHT => new_x = new_x + 1,
            }

            new_grid[self.get_grid_index(new_x, new_y)].push(element_to_move);
            for element_in_next_location in &self.grid[self.get_grid_index(new_x, new_y)] {
                if self.is_adjective(element_in_next_location, Adjective::PUSH) {
                    moves_to_do.push_back((
                        element_in_next_location.clone(),
                        new_x,
                        new_y,
                        direction_to_move,
                    ));
                }
            }
        }

        self.old_grids.push_back(self.grid.clone());
        self.grid = new_grid;

        self.build_rules();

        self.is_win()
    }

    fn can_move(&self, x: usize, y: usize, direction: &Direction) -> bool {
        let mut new_x = x;
        let mut new_y = y;
        // Objects can't go off limits
        match direction {
            Direction::UP => {
                if new_y > 0 {
                    new_y = new_y - 1
                } else {
                    return false;
                }
            }
            Direction::DOWN => {
                if new_y < self.height - 1 {
                    new_y = new_y + 1
                } else {
                    return false;
                }
            }
            Direction::LEFT => {
                if new_x > 0 {
                    new_x = new_x - 1
                } else {
                    return false;
                }
            }
            Direction::RIGHT => {
                if new_x < self.width - 1 {
                    new_x = new_x + 1
                } else {
                    return false;
                }
            }
        }

        for element_in_next_location in &self.grid[self.get_grid_index(new_x, new_y)] {
            // Can't pass stop objects
            if self.is_adjective(&element_in_next_location, Adjective::STOP) {
                return false;
            }

            // Can't move if push objects can't be pushed
            if self.is_adjective(&element_in_next_location, Adjective::PUSH) {
                if !self.can_move(new_x, new_y, direction) {
                    return false;
                }
            }
        }

        true
    }

    // This shall be called once the objects have moved (i.e. self.grid is up-to-date)
    fn build_rules(&mut self) {
        let mut new_rules: Vec<NounIsAdjectiveRule> = Vec::new();

        // Constant rules
        new_rules.push(NounIsAdjectiveRule {
            noun: Noun::TEXT,
            adjective: Adjective::PUSH,
        });

        // Vertical rules
        for x in 0..self.width {
            for y in 0..self.height - 2 {
                for el1 in &self.grid[self.get_grid_index(x, y)] {
                    for el2 in &self.grid[self.get_grid_index(x, y + 1)] {
                        for el3 in &self.grid[self.get_grid_index(x, y + 2)] {
                            if let Some(rule) = get_rule(el1, el2, el3) {
                                new_rules.push(rule);
                            }
                        }
                    }
                }
            }
        }

        // Vertical rules
        for x in 0..self.width {
            for y in 0..self.height - 2 {
                for el1 in &self.grid[self.get_grid_index(x, y)] {
                    for el2 in &self.grid[self.get_grid_index(x + 1, y)] {
                        for el3 in &self.grid[self.get_grid_index(x + 2, y)] {
                            if let Some(rule) = get_rule(el1, el2, el3) {
                                new_rules.push(rule);
                            }
                        }
                    }
                }
            }
        }

        self.rules = new_rules;
    }

    fn is_adjective(&self, element: &Element, adjective: Adjective) -> bool {
        for rule in &self.rules {
            if rule.noun == get_noun(element) && rule.adjective == adjective {
                return true;
            }
        }
        false
    }

    fn is_win(&self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                for element in &self.grid[self.get_grid_index(x, y)] {
                    if !self.is_adjective(element, Adjective::YOU) {
                        continue;
                    }

                    for element_at_same_location in &self.grid[self.get_grid_index(x, y)] {
                        if self.is_adjective(element_at_same_location, Adjective::WIN) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn print(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        // write!(stdout, "{}", termion::clear::All).unwrap();
        let mut line_number = 0;
        for y in 0..self.height {
            line_number += 1;
            let mut line = String::with_capacity(self.width);
            for x in 0..self.width {
                let first_element = self.grid[self.get_grid_index(x, y)].get(0);
                line.push_str(&get_printable_character(first_element))
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

        for rule in &self.rules {
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
}

fn main() {
    let mut level = Level::new(12, 15);
    level.add_object(2, 3, Element::Object(Object::FERRIS));
    level.add_object(1, 9, Element::Object(Object::FLAG));
    level.add_object(2, 9, Element::Object(Object::FLAG));
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
    level.add_object(7, 12, Element::Word(Word::Noun(Noun::FLAG)));
    level.add_object(8, 12, Element::Word(Word::IS));
    level.add_object(9, 12, Element::Word(Word::Adjective(Adjective::PUSH)));

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
            Key::Char('u') => level.undo(),
            Key::Backspace => level.undo(),
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
