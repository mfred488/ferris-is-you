extern crate js_sys;
mod utils;

use ferris_base::core::direction::Direction;
use ferris_base::core::level::Level;
use ferris_base::core::level::OrientedElement;
use ferris_base::unicode::build_level_from_lines;
use ferris_base::unicode::element_to_unicode;

use js_sys::Array;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct WasmLevel {
    level: Level,
}

#[wasm_bindgen]
impl WasmLevel {
    pub fn new() -> WasmLevel {
        let lines = "# shuffle
..................................................................
............................................................🚀....
..............................................................🚀..
........🚀......🚀....🍄🍄🍄🍄🍄🍄🍄🍄🍄🍄🍄........🚀............
......🚀..............🍄..................🍄......................
......................🍄......🦀..........🍄......................
......................🍄..................🍄......................
......................🍄................Sk🍄......................
......................🍄....Fe==U ......==🍄......................
......................🍄🍄🍄🍄🍄🍄......Df🍄......................
......................💀..Fg==Wi🍄........🍄......................
......🚀..............💀........🍄........🍄......................
......................💀....🚩..🍄........🍄..............🚀......
..🚀..................💀........🍄........🍄......................
......................💀💀💀💀💀🍄🍄🚀🍄🍄🍄..........🚀..........
..........🚀......................................................
............................................................🚀....
..............Fu==St.............................................."
            .lines();
        WasmLevel {
            level: build_level_from_lines(lines, None),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        self.level.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        self.level.height
    }

    pub fn next(&mut self, input: String) -> bool {
        let direction = match input.as_str() {
            "undo" => {
                self.level.undo();
                return false;
            }
            "up" => Some(Direction::UP),
            "down" => Some(Direction::DOWN),
            "right" => Some(Direction::RIGHT),
            "left" => Some(Direction::LEFT),
            _ => None,
        };
        self.level.next(direction)
    }

    pub fn get_grid(&self) -> Array {
        let mut result: Vec<Array> = Vec::new();
        for x in 0..self.level.width {
            for y in 0..self.level.height {
                let mut cell: Vec<WasmOrientedElement> = Vec::new();
                let oriented_elements = self.level.get_oriented_elements(x, y);
                for oriented_element in oriented_elements {
                    cell.push(WasmOrientedElement::new(oriented_element))
                }
                result.push(cell.into_iter().map(JsValue::from).collect())
            }
        }

        result.into_iter().map(JsValue::from).collect()
    }

    pub fn get_grid_index(&self, x: usize, y: usize) -> usize {
        self.level.get_grid_index(x, y)
    }
}

#[wasm_bindgen]
pub struct WasmOrientedElement {
    orientation: String,
    object: String,
    unicode: String,
}

#[wasm_bindgen]
impl WasmOrientedElement {
    fn new(oriented_element: &OrientedElement) -> WasmOrientedElement {
        let orientation = match &oriented_element.orientation {
            Direction::UP => "up",
            Direction::DOWN => "down",
            Direction::RIGHT => "right",
            Direction::LEFT => "left",
        };
        let object = format!("{:?}", &oriented_element.element);

        WasmOrientedElement {
            object,
            orientation: orientation.to_string(),
            unicode: element_to_unicode(Some(&oriented_element.element)).to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn orientation(&self) -> String {
        self.orientation.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn object(&self) -> String {
        self.object.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn unicode(&self) -> String {
        self.unicode.clone()
    }
}

#[wasm_bindgen]
pub fn init_level() -> WasmLevel {
    WasmLevel::new()
}
