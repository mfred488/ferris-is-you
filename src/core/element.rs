#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Object {
    FERRIS,
    ROCKET,
    FLAG,
    WALL,
    WATER,
    LAVA,
    KEY,
    DOOR,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Text {
    Nominal(Nominal),
    Verb(Verb),
    Conjunction(Conjunction),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Nominal {
    Noun(Noun),
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
    KEY,
    DOOR,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Conjunction {
    AND,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Verb {
    IS,
    HAS,
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
    SHIFT,
    OPEN,
    SHUT,
    WEAK,
    PULL,
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
        Element::Object(Object::KEY) => Noun::KEY,
        Element::Object(Object::DOOR) => Noun::DOOR,
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
        Noun::KEY => Element::Object(Object::KEY),
        Noun::DOOR => Element::Object(Object::DOOR),
        Noun::TEXT => Element::Text(Text::Nominal(Nominal::Noun(get_noun(original_element)))),
    }
}
