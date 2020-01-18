use super::direction::{get_opposite_direction, Direction};
use super::element::*;
use super::rule::{is_rule, NounIsAdjectiveRule};
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
struct OrientedElement {
    element: Element,
    orientation: Direction,
}

impl OrientedElement {
    fn new(element: Element, orientation: Direction) -> OrientedElement {
        OrientedElement {
            element,
            orientation,
        }
    }
}

pub struct Level {
    pub width: usize,
    pub height: usize,
    grid: Vec<Vec<OrientedElement>>,
    old_grids: VecDeque<Vec<Vec<OrientedElement>>>,
    pub rules: Vec<NounIsAdjectiveRule>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Level {
        assert!(width > 2 && height > 2);
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

    pub fn add_element(&mut self, x: usize, y: usize, element: Element) {
        let index = self.get_grid_index(x, y);
        self.grid[index].push(OrientedElement::new(element, Direction::DOWN));
        self.build_rules();
    }

    fn get_oriented_elements(&self, x: usize, y: usize) -> &Vec<OrientedElement> {
        &self.grid[self.get_grid_index(x, y)]
    }

    pub fn get_elements(&self, x: usize, y: usize) -> Vec<Element> {
        self.get_oriented_elements(x, y)
            .iter()
            .map(|&oel| oel.element.clone())
            .collect()
    }

    pub fn undo(&mut self) {
        match self.old_grids.pop_back() {
            Some(grid) => {
                self.grid = grid;
                self.build_rules();
            }
            None => {}
        }
    }

    pub fn next(&mut self, input: Option<Direction>) -> bool {
        let mut new_grid: Vec<Vec<OrientedElement>> = self.grid.clone();
        let mut moves_to_do: VecDeque<(Vec<Element>, usize, usize, Direction)> = VecDeque::new();

        if let Some(input_direction) = input {
            for x in 0..self.width {
                for y in 0..self.height {
                    if self.can_move(&new_grid, x, y, &input_direction) {
                        let mut elements_to_move: Vec<Element> = Vec::new();
                        for oriented_element in self.get_oriented_elements(x, y) {
                            if self.is_adjective(&oriented_element.element, Adjective::YOU) {
                                elements_to_move.push(oriented_element.element.clone());
                            }
                        }
                        if elements_to_move.len() > 0 {
                            moves_to_do.push_back((
                                elements_to_move,
                                x,
                                y,
                                input_direction.clone(),
                            ));
                        }
                    }
                }
            }
        }
        self.process_moves(&mut new_grid, moves_to_do);

        let mut moves_to_do: VecDeque<(Vec<Element>, usize, usize, Direction)> = VecDeque::new();
        for x in 0..self.width {
            for y in 0..self.height {
                for direction in [
                    Direction::UP,
                    Direction::DOWN,
                    Direction::LEFT,
                    Direction::RIGHT,
                ]
                .iter()
                {
                    let opposite_direction = get_opposite_direction(direction);
                    if self.can_move(&new_grid, x, y, direction) {
                        let mut elements_to_move: Vec<Element> = Vec::new();
                        for oriented_element in &new_grid[self.get_grid_index(x, y)] {
                            if self.is_adjective(&oriented_element.element, Adjective::MOVE)
                                && &oriented_element.orientation == direction
                            {
                                elements_to_move.push(oriented_element.element.clone());
                            }
                        }

                        if !self.can_move(&new_grid, x, y, &opposite_direction) {
                            for oriented_element in &new_grid[self.get_grid_index(x, y)] {
                                if self.is_adjective(&oriented_element.element, Adjective::MOVE)
                                    && oriented_element.orientation == opposite_direction
                                {
                                    elements_to_move.push(oriented_element.element.clone());
                                }
                            }
                        }

                        if elements_to_move.len() > 0 {
                            moves_to_do.push_back((elements_to_move, x, y, *direction));
                        }
                    }
                }
            }
        }
        self.process_moves(&mut new_grid, moves_to_do);

        for x in 0..self.width {
            for y in 0..self.height {
                let cell_has_defeat = new_grid[self.get_grid_index(x, y)]
                    .iter()
                    .find(|&oel| self.is_adjective(&oel.element, Adjective::DEFEAT))
                    .is_some();

                if cell_has_defeat {
                    // Check interaction with float (cf game trailer)
                    new_grid[self.get_grid_index(x, y)]
                        .retain(|&oel| !self.is_adjective(&oel.element, Adjective::YOU));
                }

                let cell_has_sink = new_grid[self.get_grid_index(x, y)]
                    .iter()
                    .find(|&oel| self.is_adjective(&oel.element, Adjective::SINK))
                    .is_some();

                let cell_has_non_floating_element = new_grid[self.get_grid_index(x, y)]
                    .iter()
                    .find(|&oel| {
                        !self.is_adjective(&oel.element, Adjective::SINK)
                            && !self.is_adjective(&oel.element, Adjective::FLOAT)
                    })
                    .is_some();

                if cell_has_sink && cell_has_non_floating_element {
                    new_grid[self.get_grid_index(x, y)]
                        .retain(|&oel| self.is_adjective(&oel.element, Adjective::FLOAT));
                }

                let cell_has_hot = new_grid[self.get_grid_index(x, y)]
                    .iter()
                    .find(|&oel| self.is_adjective(&oel.element, Adjective::HOT))
                    .is_some();

                if cell_has_hot {
                    new_grid[self.get_grid_index(x, y)]
                        .retain(|&oel| !self.is_adjective(&oel.element, Adjective::MELT));
                }
            }
        }

        self.old_grids.push_back(self.grid.clone());
        self.grid = new_grid;

        self.build_rules();

        self.is_win()
    }

    fn can_move(
        &self,
        grid: &Vec<Vec<OrientedElement>>,
        x: usize,
        y: usize,
        direction: &Direction,
    ) -> bool {
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

        for oriented_element_in_next_location in &grid[self.get_grid_index(new_x, new_y)] {
            // Can't pass stop objects
            if self.is_adjective(&oriented_element_in_next_location.element, Adjective::STOP) {
                return false;
            }

            // Can't move if push objects can't be pushed
            if self.is_adjective(&oriented_element_in_next_location.element, Adjective::PUSH) {
                if !self.can_move(&grid, new_x, new_y, direction) {
                    return false;
                }
            }
        }

        true
    }

    fn process_moves(
        &self,
        ongoing_grid: &mut Vec<Vec<OrientedElement>>,
        mut moves_to_do: VecDeque<(Vec<Element>, usize, usize, Direction)>,
    ) {
        while moves_to_do.len() > 0 {
            let (elements_to_move, x, y, direction_to_move) = moves_to_do.pop_front().unwrap();

            let mut new_x = x;
            let mut new_y = y;
            match direction_to_move {
                Direction::UP => new_y = new_y - 1,
                Direction::DOWN => new_y = new_y + 1,
                Direction::LEFT => new_x = new_x - 1,
                Direction::RIGHT => new_x = new_x + 1,
            }

            let mut elements_in_next_location_to_push: Vec<Element> = Vec::new();
            for oriented_element_in_next_location in
                &ongoing_grid[self.get_grid_index(new_x, new_y)]
            {
                if self.is_adjective(&oriented_element_in_next_location.element, Adjective::PUSH) {
                    elements_in_next_location_to_push
                        .push(oriented_element_in_next_location.element.clone());
                }
            }
            if elements_in_next_location_to_push.len() > 0 {
                moves_to_do.push_back((
                    elements_in_next_location_to_push,
                    new_x,
                    new_y,
                    direction_to_move,
                ));
            }

            for element_to_move in elements_to_move {
                let index_to_remove = ongoing_grid[self.get_grid_index(x, y)]
                    .iter()
                    .position(|&oel| oel.element == element_to_move)
                    .unwrap();
                ongoing_grid[self.get_grid_index(x, y)].remove(index_to_remove);

                ongoing_grid[self.get_grid_index(new_x, new_y)]
                    .push(OrientedElement::new(element_to_move, direction_to_move));
            }
        }
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
                for oel1 in self.get_oriented_elements(x, y) {
                    for oel2 in self.get_oriented_elements(x, y + 1) {
                        for oel3 in self.get_oriented_elements(x, y + 2) {
                            if let Some(rule) = is_rule(&oel1.element, &oel2.element, &oel3.element)
                            {
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
                for oel1 in self.get_oriented_elements(x, y) {
                    for oel2 in self.get_oriented_elements(x + 1, y) {
                        for oel3 in self.get_oriented_elements(x + 2, y) {
                            if let Some(rule) = is_rule(&oel1.element, &oel2.element, &oel3.element)
                            {
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
                for oriented_element in self.get_oriented_elements(x, y) {
                    if !self.is_adjective(&oriented_element.element, Adjective::YOU) {
                        continue;
                    }

                    for oriented_element_at_same_location in self.get_oriented_elements(x, y) {
                        if self.is_adjective(
                            &oriented_element_at_same_location.element,
                            Adjective::WIN,
                        ) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}
