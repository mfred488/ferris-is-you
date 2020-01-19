#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Object {
    FERRIS,
    ROCKET,
    FLAG,
    WALL,
    WATER,
    LAVA,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Text {
    Noun(Noun),
    IS,
    Adjective(Adjective),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Noun {
    FERRIS,
    ROCKET,
    FLAG,
    WALL,
    WATER,
    TEXT,
    LAVA,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Adjective {
    YOU,
    WIN,
    STOP,
    PUSH,
    SINK,
    FLOAT,
    MOVE,
    DEFEAT,
    HOT,
    MELT,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Element {
    Object(Object),
    Text(Text),
}

pub fn get_noun(element: &Element) -> Noun {
    match element {
        Element::Text(_) => Noun::TEXT,
        Element::Object(Object::FERRIS) => Noun::FERRIS,
        Element::Object(Object::ROCKET) => Noun::ROCKET,
        Element::Object(Object::FLAG) => Noun::FLAG,
        Element::Object(Object::WALL) => Noun::WALL,
        Element::Object(Object::WATER) => Noun::WATER,
        Element::Object(Object::LAVA) => Noun::LAVA,
    }
}

pub fn transform_into(original_element: &Element, noun: &Noun) -> Element {
    match noun {
        Noun::FERRIS => Element::Object(Object::FERRIS),
        Noun::ROCKET => Element::Object(Object::ROCKET),
        Noun::FLAG => Element::Object(Object::FLAG),
        Noun::WALL => Element::Object(Object::WALL),
        Noun::WATER => Element::Object(Object::WATER),
        Noun::LAVA => Element::Object(Object::LAVA),
        Noun::TEXT => Element::Text(Text::Noun(get_noun(original_element))),
    }
}
