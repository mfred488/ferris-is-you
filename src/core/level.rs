use super::direction::Direction;
use super::element::*;
use super::rule::{is_rule, NounIsAdjectiveRule};
use std::collections::VecDeque;

pub struct Level {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Element>>,
    old_grids: VecDeque<Vec<Vec<Element>>>,
    pub rules: Vec<NounIsAdjectiveRule>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Level {
        assert!(width > 3 && height > 3);
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

    pub fn get_grid_index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);
        x * self.height + y
    }

    pub fn add_element(&mut self, x: usize, y: usize, element: Element) {
        let index = self.get_grid_index(x, y);
        self.grid[index].push(element);
        self.build_rules();
    }

    pub fn undo(&mut self) {
        match self.old_grids.pop_back() {
            Some(grid) => self.grid = grid,
            None => {}
        }
    }

    pub fn next(&mut self, direction: Direction) -> bool {
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

        for x in 0..self.width {
            for y in 0..self.height {
                let cell_has_sink = new_grid[self.get_grid_index(x, y)]
                    .iter()
                    .find(|&element| self.is_adjective(element, Adjective::SINK))
                    .is_some();

                if cell_has_sink {
                    new_grid[self.get_grid_index(x, y)].retain(|&element| {
                        self.is_adjective(&element, Adjective::SINK)
                            || self.is_adjective(&element, Adjective::FLOAT)
                    });
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
                            if let Some(rule) = is_rule(el1, el2, el3) {
                                new_rules.push(rule);
                            }
                        }
                    }
                }
            }
        }

        // Vertical rules
        for x in 0..self.width - 2 {
            for y in 0..self.height {
                for el1 in &self.grid[self.get_grid_index(x, y)] {
                    for el2 in &self.grid[self.get_grid_index(x + 1, y)] {
                        for el3 in &self.grid[self.get_grid_index(x + 2, y)] {
                            if let Some(rule) = is_rule(el1, el2, el3) {
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
}
