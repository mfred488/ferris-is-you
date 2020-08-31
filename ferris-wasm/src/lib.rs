mod utils;

use ferris_base::core::direction::Direction;
use ferris_base::core::element::*;
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
pub struct WasmOrientedElement {
    orientation: String,
    object: String,
}

#[wasm_bindgen]
impl WasmOrientedElement {
    #[wasm_bindgen(getter)]
    pub fn orientation(&self) -> String {
        self.orientation.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn object(&self) -> String {
        self.object.clone()
    }
}

fn oriented_element_to_wasm(element: Element, direction: Direction) -> WasmOrientedElement {
    let orientation = match direction {
        Direction::UP => "up",
        Direction::DOWN => "down",
        Direction::RIGHT => "right",
        Direction::LEFT => "left",
    };
    let object = format!("{:?}", element);

    WasmOrientedElement {
        object,
        orientation: orientation.to_string(),
    }
}

#[wasm_bindgen]
pub fn get_sample_element() -> WasmOrientedElement {
    oriented_element_to_wasm(Element::Object(Object::ROCK), Direction::RIGHT)
}

#[wasm_bindgen]
pub fn get_sample_text() -> WasmOrientedElement {
    oriented_element_to_wasm(
        Element::Text(Text::Nominal(Nominal::Noun(Noun::FERRIS))),
        Direction::LEFT,
    )
}
