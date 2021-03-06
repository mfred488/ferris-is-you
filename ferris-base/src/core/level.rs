extern crate rand;

use super::direction::{get_opposite_direction, Direction};
use super::element::*;
use super::rule::{
    is_rule_3, is_rule_4, is_rule_5, is_rule_7, NounIsNominalRule, QualifiedNoun, Rule,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OrientedElement {
    pub element: Element,
    pub orientation: Direction,
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
    rng: StdRng,
}

impl Level {
    pub fn new(width: usize, height: usize, seed: Option<[u8; 32]>) -> Level {
        assert!(width > 2 && height > 2);
        let mut level = Level {
            width,
            height,
            grid: Vec::with_capacity(width * height),
            old_grids: VecDeque::new(),
            rules: Vec::new(),
            rng: SeedableRng::from_seed(seed.unwrap_or([0; 32])),
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
        self.grid[index].push(OrientedElement::new(element, Direction::DOWN));
        self.build_rules();
    }

    pub fn get_oriented_elements(&self, x: usize, y: usize) -> &Vec<OrientedElement> {
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

        let mut moves_to_do: VecDeque<(Vec<OrientedElement>, usize, usize, Direction, bool)> =
            VecDeque::new();
        let mut elements_to_destroy: Vec<(usize, usize, usize)> = Vec::new();
        if let Some(input_direction) = input {
            for x in 0..self.width {
                for y in 0..self.height {
                    let mut elements_to_move: Vec<OrientedElement> = Vec::new();
                    for (index, oriented_element) in
                        self.get_oriented_elements(x, y).iter().enumerate()
                    {
                        if self.is_adjective(&oriented_element, x, y, Adjective::YOU) {
                            if self.can_move(x, y, &oriented_element, &input_direction) {
                                elements_to_move.push(oriented_element.clone());
                            } else if self.is_adjective(&oriented_element, x, y, Adjective::WEAK) {
                                elements_to_destroy.push((index, x, y));
                            }
                        }
                    }
                    if elements_to_move.len() > 0 {
                        moves_to_do.push_back((
                            elements_to_move,
                            x,
                            y,
                            input_direction.clone(),
                            false,
                        ));
                    }
                }
            }
        }
        for &(index_to_destroy, x, y) in elements_to_destroy.iter().rev() {
            let grid_index = self.get_grid_index(x, y);
            let removed_element = self.grid[grid_index].remove(index_to_destroy);
            let new_elements = &mut self.destroy_element(&removed_element);
            self.grid[grid_index].append(new_elements);
        }
        self.process_moves(moves_to_do);

        let mut moves_to_do: VecDeque<(Vec<OrientedElement>, usize, usize, Direction, bool)> =
            VecDeque::new();
        let mut elements_to_destroy: Vec<(usize, usize, usize)> = Vec::new();
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
                    let mut elements_to_move: Vec<OrientedElement> = Vec::new();
                    for (index, oriented_element) in
                        self.get_oriented_elements(x, y).iter().enumerate()
                    {
                        if self.is_adjective(&oriented_element, x, y, Adjective::MOVE)
                            && &oriented_element.orientation == direction
                        {
                            if self.can_move(x, y, &oriented_element, direction) {
                                elements_to_move.push(oriented_element.clone());
                            } else if self.is_adjective(&oriented_element, x, y, Adjective::WEAK) {
                                elements_to_destroy.push((index, x, y));
                            }
                        }

                        if !self.can_move(x, y, &oriented_element, &opposite_direction)
                            && self.can_move(x, y, &oriented_element, direction)
                            && self.is_adjective(&oriented_element, x, y, Adjective::MOVE)
                            && oriented_element.orientation == opposite_direction
                            && !self.is_adjective(&oriented_element, x, y, Adjective::WEAK)
                        {
                            // If this element is weak, it won't turn around, but will be destroyed (cf above)
                            elements_to_move.push(oriented_element.clone());
                        }
                    }

                    if elements_to_move.len() > 0 {
                        moves_to_do.push_back((elements_to_move, x, y, *direction, false));
                    }
                }
            }
        }
        for &(index_to_destroy, x, y) in elements_to_destroy.iter().rev() {
            let grid_index = self.get_grid_index(x, y);
            let removed_element = self.grid[grid_index].remove(index_to_destroy);
            let new_elements = &mut self.destroy_element(&removed_element);
            self.grid[grid_index].append(new_elements);
        }
        self.process_moves(moves_to_do);
        self.cleanup();

        let mut moves_to_do: VecDeque<(Vec<OrientedElement>, usize, usize, Direction, bool)> =
            VecDeque::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let mut new_cell_orientation = None;
                let mut number_of_shift_elements = 0;
                for oriented_element in self.get_oriented_elements(x, y).iter().rev() {
                    if self.is_adjective(&oriented_element, x, y, Adjective::SHIFT) {
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
                    let grid_index = self.get_grid_index(x, y);
                    let mut elements_to_move: Vec<OrientedElement> = Vec::new();
                    for mut oriented_element in &mut self.grid[grid_index] {
                        oriented_element.orientation = new_cell_orientation.clone();
                    }

                    for oriented_element in self.get_oriented_elements(x, y) {
                        if self.can_move(x, y, &oriented_element, &new_cell_orientation) {
                            if number_of_shift_elements > 1 {
                                elements_to_move.push(oriented_element.clone());
                            } else if !self.is_adjective(&oriented_element, x, y, Adjective::SHIFT)
                            {
                                // only one shift element: this element must not move
                                elements_to_move.push(oriented_element.clone());
                            }
                        }
                    }

                    if elements_to_move.len() > 0 {
                        moves_to_do.push_back((
                            elements_to_move,
                            x,
                            y,
                            new_cell_orientation.clone(),
                            false,
                        ));
                    }
                }
            }
        }
        self.process_moves(moves_to_do);

        let mut tele_moves_to_do: Vec<((usize, usize), Vec<(usize, usize)>, OrientedElement)> =
            Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let find_tele_item_result = self
                    .get_oriented_elements(x, y)
                    .iter()
                    .find(|&oel| self.is_adjective(&oel, x, y, Adjective::TELE));
                if let Some(tele_oriented_element) = find_tele_item_result {
                    let mut targets: Vec<(usize, usize)> = Vec::new();
                    for target_x in 0..self.width {
                        for target_y in 0..self.height {
                            if target_x == x && target_y == y {
                                continue;
                            }
                            let is_eligible_tele_target = self
                                .get_oriented_elements(target_x, target_y)
                                .iter()
                                .any(|&oel| &oel.element == &tele_oriented_element.element);

                            if is_eligible_tele_target {
                                targets.push((target_x, target_y));
                            }
                        }
                    }

                    if targets.len() > 0 {
                        for oriented_element in self.get_oriented_elements(x, y) {
                            if oriented_element != tele_oriented_element {
                                tele_moves_to_do.push((
                                    (x, y),
                                    targets.clone(),
                                    oriented_element.clone(),
                                ));
                            }
                        }
                    }
                }
            }
        }
        for (from, targets, oriented_element) in tele_moves_to_do {
            let target = targets[self.rng.gen_range(0, targets.len())];
            self.move_element(
                from,
                target,
                oriented_element.element,
                oriented_element.orientation,
            );
        }
        self.cleanup();

        loop {
            let mut moves_to_do: VecDeque<(Vec<OrientedElement>, usize, usize, Direction, bool)> =
                VecDeque::new();
            for x in 0..self.width {
                for y in 0..self.height {
                    let mut elements_to_move: Vec<OrientedElement> = Vec::new();
                    for oriented_element in self.get_oriented_elements(x, y) {
                        if self.is_adjective(&oriented_element, x, y, Adjective::FALL)
                            && self.can_move(x, y, &oriented_element, &Direction::DOWN)
                        {
                            elements_to_move.push(oriented_element.clone())
                        }
                    }

                    if elements_to_move.len() > 0 {
                        moves_to_do.push_back((elements_to_move, x, y, Direction::DOWN, true));
                    }
                }
            }
            if moves_to_do.len() == 0 {
                break;
            }
            self.process_moves(moves_to_do);
        }

        self.cleanup();
        self.build_rules();
        self.cleanup();
        self.transform_elements();

        self.is_win()
    }

    fn can_move(
        &self,
        x: usize,
        y: usize,
        oriented_element: &OrientedElement,
        direction: &Direction,
    ) -> bool {
        if let Some((new_x, new_y)) = self.get_next_location(x, y, &direction) {
            if self.is_adjective(&oriented_element, x, y, Adjective::SWAP) {
                return true;
            }
            for oriented_element_in_next_location in self.get_oriented_elements(new_x, new_y) {
                // Can always swap
                if self.is_adjective(
                    &oriented_element_in_next_location,
                    new_x,
                    new_y,
                    Adjective::SWAP,
                ) {
                    return true;
                }

                // Can't pass stop objects
                if self.is_adjective(
                    &oriented_element_in_next_location,
                    new_x,
                    new_y,
                    Adjective::STOP,
                ) {
                    return false;
                }

                if self.is_adjective(
                    &oriented_element_in_next_location,
                    new_x,
                    new_y,
                    Adjective::PUSH,
                ) {
                    // Can't move if push objects can't be pushed
                    if !self.can_move(new_x, new_y, &oriented_element_in_next_location, direction) {
                        return false;
                    }
                } else if self.is_adjective(
                    &oriented_element_in_next_location,
                    new_x,
                    new_y,
                    Adjective::PULL,
                ) {
                    // Can't move if there's a non-pushable pulled object
                    return false;
                }
            }

            true
        } else {
            // Objects can't go off limits
            false
        }
    }

    fn get_next_location(
        &self,
        x: usize,
        y: usize,
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        let mut new_x = x;
        let mut new_y = y;
        match direction {
            Direction::UP => {
                if new_y > 0 {
                    new_y = new_y - 1;
                } else {
                    return None;
                }
            }
            Direction::DOWN => {
                if new_y < self.height - 1 {
                    new_y = new_y + 1;
                } else {
                    return None;
                }
            }
            Direction::LEFT => {
                if new_x > 0 {
                    new_x = new_x - 1;
                } else {
                    return None;
                }
            }
            Direction::RIGHT => {
                if new_x < self.width - 1 {
                    new_x = new_x + 1;
                } else {
                    return None;
                }
            }
        }

        Some((new_x, new_y))
    }

    fn process_moves(
        &mut self,
        mut moves_to_do: VecDeque<(Vec<OrientedElement>, usize, usize, Direction, bool)>,
    ) {
        while moves_to_do.len() > 0 {
            let (oriented_elements_to_move, x, y, direction_to_move, keep_original_orientation) =
                moves_to_do.pop_front().unwrap();

            let mut new_x = x;
            let mut new_y = y;
            match direction_to_move {
                Direction::UP => new_y = new_y - 1,
                Direction::DOWN => new_y = new_y + 1,
                Direction::LEFT => new_x = new_x - 1,
                Direction::RIGHT => new_x = new_x + 1,
            }
            let opposite_direction = get_opposite_direction(&direction_to_move);

            let should_swap_all = oriented_elements_to_move
                .iter()
                .any(|&el| self.is_adjective(&el, x, y, Adjective::SWAP));

            let mut elements_in_next_location_to_swap_with: Vec<OrientedElement> = Vec::new();
            let mut elements_in_next_location_to_push: Vec<OrientedElement> = Vec::new();
            for oriented_element_in_next_location in self.get_oriented_elements(new_x, new_y) {
                if should_swap_all
                    || self.is_adjective(
                        &oriented_element_in_next_location,
                        new_x,
                        new_y,
                        Adjective::SWAP,
                    )
                {
                    elements_in_next_location_to_swap_with
                        .push(oriented_element_in_next_location.clone());
                } else if self.is_adjective(
                    &oriented_element_in_next_location,
                    new_x,
                    new_y,
                    Adjective::PUSH,
                ) {
                    elements_in_next_location_to_push
                        .push(oriented_element_in_next_location.clone());
                }
            }
            if elements_in_next_location_to_swap_with.len() > 0 {
                moves_to_do.push_back((
                    elements_in_next_location_to_swap_with,
                    new_x,
                    new_y,
                    opposite_direction,
                    true,
                ));
            }
            if elements_in_next_location_to_push.len() > 0 {
                moves_to_do.push_back((
                    elements_in_next_location_to_push,
                    new_x,
                    new_y,
                    direction_to_move,
                    false,
                ));
            }

            if let Some((pulled_x, pulled_y)) = self.get_next_location(x, y, &opposite_direction) {
                let elements_to_pull: Vec<OrientedElement> = self
                    .get_oriented_elements(pulled_x, pulled_y)
                    .iter()
                    .filter(|&oel| self.is_adjective(&oel, pulled_x, pulled_y, Adjective::PULL))
                    .map(|&oel| oel.clone())
                    .collect();

                if elements_to_pull.len() > 0 {
                    moves_to_do.push_back((
                        elements_to_pull,
                        pulled_x,
                        pulled_y,
                        direction_to_move.clone(),
                        false,
                    ));
                }
            }

            for element_to_move in oriented_elements_to_move {
                self.move_element(
                    (x, y),
                    (new_x, new_y),
                    element_to_move.element,
                    if keep_original_orientation {
                        element_to_move.orientation
                    } else {
                        direction_to_move
                    },
                );
            }
        }
    }

    fn move_element(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        element: Element,
        orientation: Direction,
    ) {
        let index_to_remove = self
            .get_oriented_elements(from.0, from.1)
            .iter()
            .position(|&oel| oel.element == element)
            .unwrap();
        let old_index = self.get_grid_index(from.0, from.1);
        let new_index = self.get_grid_index(to.0, to.1);
        self.grid[old_index].remove(index_to_remove);
        self.grid[new_index].push(OrientedElement::new(element, orientation));
    }

    fn cleanup(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut floating_oriented_elements: Vec<OrientedElement> = self
                    .get_oriented_elements(x, y)
                    .iter()
                    .filter(|&oel| self.is_adjective(&oel, x, y, Adjective::FLOAT))
                    .cloned()
                    .collect();
                let mut non_floating_oriented_elements: Vec<OrientedElement> = self
                    .get_oriented_elements(x, y)
                    .iter()
                    .filter(|&oel| !self.is_adjective(&oel, x, y, Adjective::FLOAT))
                    .cloned()
                    .collect();

                for oriented_elements in vec![
                    &mut floating_oriented_elements,
                    &mut non_floating_oriented_elements,
                ] {
                    let mut cell_has_sink = false;
                    let mut cell_has_not_sink = false;
                    let mut cell_has_defeat = false;
                    let mut cell_has_hot = false;
                    let mut cell_has_open = false;
                    let mut cell_has_shut = false;

                    for oel in oriented_elements.iter() {
                        cell_has_sink =
                            cell_has_sink || self.is_adjective(&oel, x, y, Adjective::SINK);
                        cell_has_not_sink =
                            cell_has_not_sink || !self.is_adjective(&oel, x, y, Adjective::SINK);
                        cell_has_defeat =
                            cell_has_defeat || self.is_adjective(&oel, x, y, Adjective::DEFEAT);
                        cell_has_hot =
                            cell_has_hot || self.is_adjective(&oel, x, y, Adjective::HOT);
                        cell_has_open =
                            cell_has_open || self.is_adjective(&oel, x, y, Adjective::OPEN);
                        cell_has_shut =
                            cell_has_shut || self.is_adjective(&oel, x, y, Adjective::SHUT);
                    }

                    let mut indexes_to_remove: Vec<usize> = Vec::new();
                    let mut new_elements: Vec<OrientedElement> = Vec::new();
                    let mut index = 0;
                    for oel in oriented_elements.iter() {
                        let must_be_destroyed = (cell_has_sink && cell_has_not_sink)
                            || (cell_has_defeat && self.is_adjective(&oel, x, y, Adjective::YOU))
                            || (cell_has_hot && self.is_adjective(&oel, x, y, Adjective::MELT))
                            || (cell_has_open
                                && cell_has_shut
                                && (self.is_adjective(&oel, x, y, Adjective::OPEN)
                                    || self.is_adjective(&oel, x, y, Adjective::SHUT)))
                            || (oriented_elements.len() > 1
                                && self.is_adjective(&oel, x, y, Adjective::WEAK));

                        if must_be_destroyed {
                            indexes_to_remove.push(index);
                            new_elements.append(&mut self.destroy_element(oel));
                        }
                        index += 1;
                    }

                    for &index_to_remove in indexes_to_remove.iter().rev() {
                        oriented_elements.remove(index_to_remove);
                    }
                    oriented_elements.append(&mut new_elements);
                }

                let index = self.get_grid_index(x, y);
                floating_oriented_elements.append(&mut non_floating_oriented_elements);
                self.grid[index] = floating_oriented_elements;
            }
        }
    }

    fn destroy_element(&self, destroyed_element: &OrientedElement) -> Vec<OrientedElement> {
        for rule in &self.rules {
            match rule {
                Rule::NounIsNominalRule(_) => {}
                Rule::NounsGroupIsNominalsGroupRule(_) => {}
                Rule::NounOnNounsGroupIsNominalsGroupRule(_) => {}
                Rule::QualifiedNounIsNominalsGroupRule(_) => {}
                Rule::NounHasNounsRule(noun_has_nouns_rule) => {
                    if noun_has_nouns_rule.subject == get_noun(&destroyed_element.element) {
                        let mut result: Vec<OrientedElement> =
                            Vec::with_capacity(noun_has_nouns_rule.objects.len());
                        for object in &noun_has_nouns_rule.objects {
                            result.push(OrientedElement {
                                element: transform_into(&destroyed_element.element, &object),
                                orientation: destroyed_element.orientation.clone(),
                            });
                        }
                        return result;
                    }
                }
            }
        }

        Vec::new()
    }

    // This shall be called once the objects have moved (i.e. self.grid is up-to-date)
    // TODO The code below starts to be pretty redundant; find a way to support variable-length rules dynamically (generated code / rust macros ? or at runtime ?)
    fn build_rules(&mut self) {
        let mut new_rules: Vec<Rule> = Vec::new();

        // Constant rules
        new_rules.push(Rule::NounIsNominalRule(NounIsNominalRule {
            noun: Noun::TEXT,
            nominal: Nominal::Adjective(Adjective::PUSH),
        }));

        // Vertical rules
        for x in 0..self.width {
            let mut y = 0;
            while y < self.height - 2 {
                if y + 7 < self.height {
                    let mut rules_7 = self.look_for_rule_7(
                        (x, y),
                        (x, y + 1),
                        (x, y + 2),
                        (x, y + 3),
                        (x, y + 4),
                        (x, y + 5),
                        (x, y + 6),
                    );
                    if !rules_7.is_empty() {
                        new_rules.append(&mut rules_7);
                        y += 7;
                        continue;
                    }
                }

                if y + 5 < self.height {
                    let mut rules_5 = self.look_for_rule_5(
                        (x, y),
                        (x, y + 1),
                        (x, y + 2),
                        (x, y + 3),
                        (x, y + 4),
                    );
                    if !rules_5.is_empty() {
                        new_rules.append(&mut rules_5);
                        y += 5;
                        continue;
                    }
                }

                if y + 4 < self.height {
                    let mut rules_4 =
                        self.look_for_rule_4((x, y), (x, y + 1), (x, y + 2), (x, y + 3));
                    if !rules_4.is_empty() {
                        new_rules.append(&mut rules_4);
                        y += 4;
                        continue;
                    }
                }
                let mut rules_3 = self.look_for_rule_3((x, y), (x, y + 1), (x, y + 2));
                if !rules_3.is_empty() {
                    new_rules.append(&mut rules_3);
                    y += 3;
                    continue;
                }
                y += 1;
            }
        }

        // Horizontal rules
        for y in 0..self.height {
            let mut x = 0;
            while x < self.width - 2 {
                if x + 7 < self.width {
                    let mut rules_7 = self.look_for_rule_7(
                        (x, y),
                        (x + 1, y),
                        (x + 2, y),
                        (x + 3, y),
                        (x + 4, y),
                        (x + 5, y),
                        (x + 6, y),
                    );
                    if !rules_7.is_empty() {
                        new_rules.append(&mut rules_7);
                        x += 7;
                        continue;
                    }
                }

                if x + 5 < self.width {
                    let mut rules_5 = self.look_for_rule_5(
                        (x, y),
                        (x + 1, y),
                        (x + 2, y),
                        (x + 3, y),
                        (x + 4, y),
                    );
                    if !rules_5.is_empty() {
                        new_rules.append(&mut rules_5);
                        x += 5;
                        continue;
                    }
                }

                if x + 4 < self.width {
                    let mut rules_4 =
                        self.look_for_rule_4((x, y), (x + 1, y), (x + 2, y), (x + 3, y));
                    if !rules_4.is_empty() {
                        new_rules.append(&mut rules_4);
                        x += 4;
                        continue;
                    }
                }
                let mut rules_3 = self.look_for_rule_3((x, y), (x + 1, y), (x + 2, y));
                if !rules_3.is_empty() {
                    new_rules.append(&mut rules_3);
                    x += 3;
                    continue;
                }
                x += 1;
            }
        }

        self.rules = new_rules;
    }

    fn look_for_rule_3(
        &self,
        p1: (usize, usize),
        p2: (usize, usize),
        p3: (usize, usize),
    ) -> Vec<Rule> {
        let mut results: Vec<Rule> = Vec::new();
        for el1 in self.get_oriented_elements(p1.0, p1.1) {
            for el2 in self.get_oriented_elements(p2.0, p2.1) {
                for el3 in self.get_oriented_elements(p3.0, p3.1) {
                    if let Some(rule) = is_rule_3(&el1.element, &el2.element, &el3.element) {
                        results.push(rule);
                    }
                }
            }
        }

        results
    }

    fn look_for_rule_4(
        &self,
        p1: (usize, usize),
        p2: (usize, usize),
        p3: (usize, usize),
        p4: (usize, usize),
    ) -> Vec<Rule> {
        let mut results: Vec<Rule> = Vec::new();
        for el1 in self.get_oriented_elements(p1.0, p1.1) {
            for el2 in self.get_oriented_elements(p2.0, p2.1) {
                for el3 in self.get_oriented_elements(p3.0, p3.1) {
                    for el4 in self.get_oriented_elements(p4.0, p4.1) {
                        if let Some(rule) =
                            is_rule_4(&el1.element, &el2.element, &el3.element, &el4.element)
                        {
                            results.push(rule);
                        }
                    }
                }
            }
        }

        results
    }

    fn look_for_rule_5(
        &self,
        p1: (usize, usize),
        p2: (usize, usize),
        p3: (usize, usize),
        p4: (usize, usize),
        p5: (usize, usize),
    ) -> Vec<Rule> {
        let mut results: Vec<Rule> = Vec::new();
        for el1 in self.get_oriented_elements(p1.0, p1.1) {
            for el2 in self.get_oriented_elements(p2.0, p2.1) {
                for el3 in self.get_oriented_elements(p3.0, p3.1) {
                    for el4 in self.get_oriented_elements(p4.0, p4.1) {
                        for el5 in self.get_oriented_elements(p5.0, p5.1) {
                            if let Some(rule) = is_rule_5(
                                &el1.element,
                                &el2.element,
                                &el3.element,
                                &el4.element,
                                &el5.element,
                            ) {
                                results.push(rule);
                            }
                        }
                    }
                }
            }
        }

        results
    }

    fn look_for_rule_7(
        &self,
        p1: (usize, usize),
        p2: (usize, usize),
        p3: (usize, usize),
        p4: (usize, usize),
        p5: (usize, usize),
        p6: (usize, usize),
        p7: (usize, usize),
    ) -> Vec<Rule> {
        let mut results: Vec<Rule> = Vec::new();
        for el1 in self.get_oriented_elements(p1.0, p1.1) {
            for el2 in self.get_oriented_elements(p2.0, p2.1) {
                for el3 in self.get_oriented_elements(p3.0, p3.1) {
                    for el4 in self.get_oriented_elements(p4.0, p4.1) {
                        for el5 in self.get_oriented_elements(p5.0, p5.1) {
                            for el6 in self.get_oriented_elements(p6.0, p6.1) {
                                for el7 in self.get_oriented_elements(p7.0, p7.1) {
                                    if let Some(rule) = is_rule_7(
                                        &el1.element,
                                        &el2.element,
                                        &el3.element,
                                        &el4.element,
                                        &el5.element,
                                        &el6.element,
                                        &el7.element,
                                    ) {
                                        results.push(rule);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        results
    }

    fn is_adjective(
        &self,
        oriented_element: &OrientedElement,
        x: usize,
        y: usize,
        adjective: Adjective,
    ) -> bool {
        for rule in &self.rules {
            match rule {
                Rule::NounIsNominalRule(noun_is_nominal_rule) => {
                    match noun_is_nominal_rule.nominal {
                        Nominal::Adjective(rule_adjective) => {
                            if noun_is_nominal_rule.noun == get_noun(&oriented_element.element)
                                && rule_adjective == adjective
                            {
                                return true;
                            }
                        }
                        _ => {}
                    }
                }
                Rule::NounsGroupIsNominalsGroupRule(nouns_is_nominals_rule) => {
                    let is_element_among_subjects = nouns_is_nominals_rule
                        .nouns
                        .iter()
                        .find(|&subject| subject == &get_noun(&oriented_element.element))
                        .is_some();
                    let is_adjective_among_nominals = nouns_is_nominals_rule
                        .nominals
                        .iter()
                        .find(|&nominal| match nominal {
                            Nominal::Adjective(local_adjective) => local_adjective == &adjective,
                            _ => false,
                        })
                        .is_some();
                    if is_element_among_subjects && is_adjective_among_nominals {
                        return true;
                    }
                }
                Rule::NounOnNounsGroupIsNominalsGroupRule(noun_on_nouns_group_is_nominals_rule) => {
                    let is_adjective_among_nominals = noun_on_nouns_group_is_nominals_rule
                        .nominals
                        .iter()
                        .find(|&nominal| match nominal {
                            Nominal::Adjective(local_adjective) => local_adjective == &adjective,
                            _ => false,
                        })
                        .is_some();

                    if noun_on_nouns_group_is_nominals_rule.subject
                        == get_noun(&oriented_element.element)
                        && is_adjective_among_nominals
                    {
                        let mut are_underlying_elements_presents = true;
                        for underlying_noun in
                            &noun_on_nouns_group_is_nominals_rule.underlying_nouns
                        {
                            let is_underlying_element_present = self
                                .get_oriented_elements(x, y)
                                .iter()
                                .find(|&oel| underlying_noun == &get_noun(&oel.element))
                                .is_some();
                            are_underlying_elements_presents &= is_underlying_element_present;

                            if !is_underlying_element_present {
                                break;
                            }
                        }

                        if are_underlying_elements_presents {
                            return true;
                        }
                    }
                }
                Rule::QualifiedNounIsNominalsGroupRule(qualified_noun_is_nominals_group_rule) => {
                    let is_adjective_among_nominals = qualified_noun_is_nominals_group_rule
                        .nominals
                        .iter()
                        .find(|&nominal| match nominal {
                            Nominal::Adjective(local_adjective) => local_adjective == &adjective,
                            _ => false,
                        })
                        .is_some();

                    if is_adjective_among_nominals {
                        match qualified_noun_is_nominals_group_rule.qualified_noun {
                            QualifiedNoun::SimpleNoun(_) => {
                                // TODO Refactor NounIsNominalGroupRule here
                            }
                            QualifiedNoun::NounNearNoun { subject, near_noun } => {
                                if subject == get_noun(&oriented_element.element) {
                                    for direction in [
                                        Direction::UP,
                                        Direction::DOWN,
                                        Direction::LEFT,
                                        Direction::RIGHT,
                                    ]
                                    .iter()
                                    {
                                        if let Some((neighbour_x, neighbour_y)) =
                                            self.get_next_location(x, y, &direction)
                                        {
                                            if self
                                                .get_oriented_elements(neighbour_x, neighbour_y)
                                                .iter()
                                                .find(|&oel| near_noun == get_noun(&oel.element))
                                                .is_some()
                                            {
                                                return true;
                                            }
                                        }
                                    }
                                }
                            }
                            QualifiedNoun::NounFacingNoun {
                                subject,
                                facing_noun,
                            } => {
                                if subject == get_noun(&oriented_element.element) {
                                    if let Some((faced_x, faced_y)) =
                                        self.get_next_location(x, y, &oriented_element.orientation)
                                    {
                                        if self
                                            .get_oriented_elements(faced_x, faced_y)
                                            .iter()
                                            .find(|&oel| facing_noun == get_noun(&oel.element))
                                            .is_some()
                                        {
                                            return true;
                                        }
                                    }
                                }
                            }
                            QualifiedNoun::LonelyNoun(noun) => {
                                if noun == get_noun(&oriented_element.element)
                                    && self.get_oriented_elements(x, y).len() == 1
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
                Rule::NounHasNounsRule(_) => {}
            }
        }
        false
    }

    fn transform_element(
        &self,
        oriented_element: OrientedElement,
        x: usize,
        y: usize,
    ) -> Vec<Element> {
        for rule in &self.rules {
            match rule {
                Rule::NounIsNominalRule(noun_is_nominal_rule) => {
                    match noun_is_nominal_rule.nominal {
                        Nominal::Noun(target_noun) => {
                            if noun_is_nominal_rule.noun == get_noun(&oriented_element.element) {
                                return vec![transform_into(
                                    &oriented_element.element,
                                    &target_noun,
                                )];
                            }
                        }
                        _ => {}
                    }
                }
                Rule::NounsGroupIsNominalsGroupRule(nouns_is_nominals_rule) => {
                    let is_element_among_subjects = nouns_is_nominals_rule
                        .nouns
                        .iter()
                        .find(|&subject| subject == &get_noun(&oriented_element.element))
                        .is_some();

                    if is_element_among_subjects {
                        let mut result: Vec<Element> = Vec::new();
                        for nominal in &nouns_is_nominals_rule.nominals {
                            match nominal {
                                Nominal::Noun(target_noun) => {
                                    result.push(transform_into(
                                        &oriented_element.element,
                                        &target_noun,
                                    ));
                                }
                                _ => {}
                            }
                        }

                        if !result.is_empty() {
                            return result;
                        }
                    }
                }
                Rule::NounOnNounsGroupIsNominalsGroupRule(noun_on_nouns_group_is_nominals_rule) => {
                    if noun_on_nouns_group_is_nominals_rule.subject
                        == get_noun(&oriented_element.element)
                    {
                        let mut are_underlying_elements_presents = true;
                        for underlying_noun in
                            &noun_on_nouns_group_is_nominals_rule.underlying_nouns
                        {
                            let is_underlying_element_present = self
                                .get_oriented_elements(x, y)
                                .iter()
                                .find(|&oel| underlying_noun == &get_noun(&oel.element))
                                .is_some();
                            are_underlying_elements_presents &= is_underlying_element_present;
                        }

                        if are_underlying_elements_presents {
                            let mut result: Vec<Element> = Vec::new();
                            for nominal in &noun_on_nouns_group_is_nominals_rule.nominals {
                                match nominal {
                                    Nominal::Noun(target_noun) => {
                                        result.push(transform_into(
                                            &oriented_element.element,
                                            &target_noun,
                                        ));
                                    }
                                    _ => {}
                                }
                            }

                            if !result.is_empty() {
                                return result;
                            }
                        }
                    }
                }
                Rule::QualifiedNounIsNominalsGroupRule(qualified_noun_is_nominals_group_rule) => {
                    let mut is_qualification_met = false;
                    match qualified_noun_is_nominals_group_rule.qualified_noun {
                        QualifiedNoun::SimpleNoun(_) => {
                            // TODO Refactor NounIsNominalsGroupRule here
                        }
                        QualifiedNoun::NounNearNoun { subject, near_noun } => {
                            if subject == get_noun(&oriented_element.element) {
                                for direction in [
                                    Direction::UP,
                                    Direction::DOWN,
                                    Direction::LEFT,
                                    Direction::RIGHT,
                                ]
                                .iter()
                                {
                                    if let Some((neighbour_x, neighbour_y)) =
                                        self.get_next_location(x, y, &direction)
                                    {
                                        if self
                                            .get_oriented_elements(neighbour_x, neighbour_y)
                                            .iter()
                                            .find(|&oel| near_noun == get_noun(&oel.element))
                                            .is_some()
                                        {
                                            is_qualification_met = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        QualifiedNoun::NounFacingNoun {
                            subject,
                            facing_noun,
                        } => {
                            if subject == get_noun(&oriented_element.element) {
                                if let Some((faced_x, faced_y)) =
                                    self.get_next_location(x, y, &oriented_element.orientation)
                                {
                                    if self
                                        .get_oriented_elements(faced_x, faced_y)
                                        .iter()
                                        .find(|&oel| facing_noun == get_noun(&oel.element))
                                        .is_some()
                                    {
                                        is_qualification_met = true;
                                    }
                                }
                            }
                        }
                        QualifiedNoun::LonelyNoun(noun) => {
                            is_qualification_met = noun == get_noun(&oriented_element.element)
                                && self.get_oriented_elements(x, y).len() == 1;
                        }
                    };

                    if is_qualification_met {
                        let mut result: Vec<Element> = Vec::new();
                        for nominal in &qualified_noun_is_nominals_group_rule.nominals {
                            match nominal {
                                Nominal::Noun(target_noun) => {
                                    result.push(transform_into(
                                        &oriented_element.element,
                                        &target_noun,
                                    ));
                                }
                                _ => {}
                            }
                        }

                        if !result.is_empty() {
                            return result;
                        }
                    }
                }
                Rule::NounHasNounsRule(_) => {}
            }
        }
        vec![oriented_element.element]
    }

    fn transform_elements(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut oriented_elements: Vec<OrientedElement> =
                    Vec::with_capacity(self.get_oriented_elements(x, y).len());

                for &oriented_element in self.get_oriented_elements(x, y) {
                    for transformed_element in self.transform_element(oriented_element, x, y) {
                        oriented_elements.push(OrientedElement {
                            element: transformed_element,
                            orientation: oriented_element.orientation,
                        })
                    }
                }
                let index = self.get_grid_index(x, y);
                self.grid[index] = oriented_elements;
            }
        }
    }

    fn is_win(&self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                for oriented_element in self.get_oriented_elements(x, y) {
                    if !self.is_adjective(&oriented_element, x, y, Adjective::YOU) {
                        continue;
                    }

                    let is_float = self.is_adjective(&oriented_element, x, y, Adjective::FLOAT);

                    for oriented_element_at_same_location in self.get_oriented_elements(x, y) {
                        if self.is_adjective(
                            &oriented_element_at_same_location,
                            x,
                            y,
                            Adjective::WIN,
                        ) && self.is_adjective(
                            &oriented_element_at_same_location,
                            x,
                            y,
                            Adjective::FLOAT,
                        ) == is_float
                        {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}
