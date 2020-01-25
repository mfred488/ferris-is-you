use super::direction::{get_opposite_direction, Direction};
use super::element::*;
use super::rule::{is_rule, NounIsAdjectiveRule, Rule};
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq)]
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
    pub rules: Vec<Rule>,
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
        self.old_grids.push_back(self.grid.clone());
        let mut moves_to_do: VecDeque<(Vec<Element>, usize, usize, Direction)> = VecDeque::new();

        if let Some(input_direction) = input {
            for x in 0..self.width {
                for y in 0..self.height {
                    if self.can_move(x, y, &input_direction) {
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
        self.process_moves(moves_to_do);

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
                    if self.can_move(x, y, direction) {
                        let mut elements_to_move: Vec<Element> = Vec::new();
                        for oriented_element in self.get_oriented_elements(x, y) {
                            if self.is_adjective(&oriented_element.element, Adjective::MOVE)
                                && &oriented_element.orientation == direction
                            {
                                elements_to_move.push(oriented_element.element.clone());
                            }
                        }

                        if !self.can_move(x, y, &opposite_direction) {
                            for oriented_element in self.get_oriented_elements(x, y) {
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
        self.process_moves(moves_to_do);

        let mut moves_to_do: VecDeque<(Vec<Element>, usize, usize, Direction)> = VecDeque::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let mut new_cell_orientation = None;
                let mut number_of_shift_elements = 0;
                for oriented_element in self.get_oriented_elements(x, y).iter().rev() {
                    if self.is_adjective(&oriented_element.element, Adjective::SHIFT) {
                        number_of_shift_elements += 1;
                        match new_cell_orientation {
                            None => {
                                new_cell_orientation = Some(oriented_element.orientation.clone())
                            }
                            Some(_) => {}
                        }
                    }
                }

                if let Some(new_cell_orientation) = new_cell_orientation {
                    // TODO check if we can just replace the orientation instead of cloning the whole thing
                    let mut new_oriented_elements: Vec<OrientedElement> =
                        Vec::with_capacity(self.get_oriented_elements(x, y).len());
                    for oriented_element in self.get_oriented_elements(x, y) {
                        new_oriented_elements.push(OrientedElement {
                            element: oriented_element.element.clone(),
                            orientation: new_cell_orientation.clone(),
                        })
                    }

                    let grid_index = self.get_grid_index(x, y);
                    self.grid[grid_index] = new_oriented_elements;
                    if self.can_move(x, y, &new_cell_orientation) {
                        let mut elements_to_move: Vec<Element> = Vec::new();
                        for oriented_element in self.get_oriented_elements(x, y) {
                            if number_of_shift_elements > 1 {
                                elements_to_move.push(oriented_element.element.clone());
                            } else if !self
                                .is_adjective(&oriented_element.element, Adjective::SHIFT)
                            {
                                // only one shift element: this element must not move
                                elements_to_move.push(oriented_element.element.clone());
                            }
                        }
                        if elements_to_move.len() > 0 {
                            moves_to_do.push_back((
                                elements_to_move,
                                x,
                                y,
                                new_cell_orientation.clone(),
                            ));
                        }
                    }
                }
            }
        }
        self.process_moves(moves_to_do);

        self.cleanup();
        self.build_rules();
        self.cleanup();

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

        for oriented_element_in_next_location in self.get_oriented_elements(new_x, new_y) {
            // Can't pass stop objects
            if self.is_adjective(&oriented_element_in_next_location.element, Adjective::STOP) {
                return false;
            }

            // Can't move if push objects can't be pushed
            if self.is_adjective(&oriented_element_in_next_location.element, Adjective::PUSH) {
                if !self.can_move(new_x, new_y, direction) {
                    return false;
                }
            }
        }

        true
    }

    fn process_moves(
        &mut self,
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
            for oriented_element_in_next_location in self.get_oriented_elements(new_x, new_y) {
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
                let index_to_remove = self
                    .get_oriented_elements(x, y)
                    .iter()
                    .position(|&oel| oel.element == element_to_move)
                    .unwrap();
                let old_index = self.get_grid_index(x, y);
                let new_index = self.get_grid_index(new_x, new_y);
                self.grid[old_index].remove(index_to_remove);

                self.grid[new_index].push(OrientedElement::new(element_to_move, direction_to_move));
            }
        }
    }

    fn cleanup(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut oriented_elements: Vec<OrientedElement> =
                    Vec::with_capacity(self.get_oriented_elements(x, y).len());

                for oriented_element in self.get_oriented_elements(x, y) {
                    oriented_elements.push(OrientedElement {
                        element: self.transform_element(oriented_element.element),
                        orientation: oriented_element.orientation,
                    })
                }

                let cell_has_defeat = oriented_elements
                    .iter()
                    .find(|&oel| self.is_adjective(&oel.element, Adjective::DEFEAT))
                    .is_some();

                if cell_has_defeat {
                    // TODO Check interaction with float (cf game trailer)
                    oriented_elements
                        .retain(|&oel| !self.is_adjective(&oel.element, Adjective::YOU));
                }

                let cell_has_sink = oriented_elements
                    .iter()
                    .find(|&oel| self.is_adjective(&oel.element, Adjective::SINK))
                    .is_some();

                let cell_has_non_floating_element = oriented_elements
                    .iter()
                    .find(|&oel| {
                        !self.is_adjective(&oel.element, Adjective::SINK)
                            && !self.is_adjective(&oel.element, Adjective::FLOAT)
                    })
                    .is_some();

                if cell_has_sink && cell_has_non_floating_element {
                    oriented_elements
                        .retain(|&oel| self.is_adjective(&oel.element, Adjective::FLOAT));
                }

                let cell_has_hot = oriented_elements
                    .iter()
                    .find(|&oel| self.is_adjective(&oel.element, Adjective::HOT))
                    .is_some();

                if cell_has_hot {
                    oriented_elements
                        .retain(|&oel| !self.is_adjective(&oel.element, Adjective::MELT));
                }

                let index = self.get_grid_index(x, y);
                self.grid[index] = oriented_elements;
            }
        }
    }

    // This shall be called once the objects have moved (i.e. self.grid is up-to-date)
    fn build_rules(&mut self) {
        let mut new_rules: Vec<Rule> = Vec::new();

        // Constant rules
        new_rules.push(Rule::NounIsAdjectiveRule(NounIsAdjectiveRule {
            noun: Noun::TEXT,
            adjective: Adjective::PUSH,
        }));

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
            match rule {
                Rule::NounIsAdjectiveRule(noun_is_adjective_rule) => {
                    if noun_is_adjective_rule.noun == get_noun(element)
                        && noun_is_adjective_rule.adjective == adjective
                    {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn transform_element(&self, element: Element) -> Element {
        for rule in &self.rules {
            match rule {
                Rule::NounIsNounRule(noun_is_noun_rule) => {
                    if noun_is_noun_rule.left == get_noun(&element) {
                        return transform_into(&element, &noun_is_noun_rule.right);
                    }
                }
                _ => {}
            }
        }
        element
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
