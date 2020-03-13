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
    MOON,
    STAR,
    BAT,
    HAND,
    FUNGUS,
    KEKE,
    ICE,
    SKULL,
    LOVE,
    BOX,
    CLIFF,
    GHOST,
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
    MOON,
    STAR,
    BAT,
    HAND,
    FUNGUS,
    KEKE,
    ICE,
    SKULL,
    LOVE,
    BOX,
    CLIFF,
    GHOST,
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
    TELE,
    SWAP,
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
        Element::Object(Object::MOON) => Noun::MOON,
        Element::Object(Object::STAR) => Noun::STAR,
        Element::Object(Object::BAT) => Noun::BAT,
        Element::Object(Object::HAND) => Noun::HAND,
        Element::Object(Object::FUNGUS) => Noun::FUNGUS,
        Element::Object(Object::KEKE) => Noun::KEKE,
        Element::Object(Object::ICE) => Noun::ICE,
        Element::Object(Object::SKULL) => Noun::SKULL,
        Element::Object(Object::LOVE) => Noun::LOVE,
        Element::Object(Object::BOX) => Noun::BOX,
        Element::Object(Object::CLIFF) => Noun::CLIFF,
        Element::Object(Object::GHOST) => Noun::GHOST,
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
        Noun::MOON => Element::Object(Object::MOON),
        Noun::STAR => Element::Object(Object::STAR),
        Noun::BAT => Element::Object(Object::BAT),
        Noun::HAND => Element::Object(Object::HAND),
        Noun::FUNGUS => Element::Object(Object::FUNGUS),
        Noun::KEKE => Element::Object(Object::KEKE),
        Noun::ICE => Element::Object(Object::ICE),
        Noun::SKULL => Element::Object(Object::SKULL),
        Noun::LOVE => Element::Object(Object::LOVE),
        Noun::BOX => Element::Object(Object::BOX),
        Noun::CLIFF => Element::Object(Object::CLIFF),
        Noun::GHOST => Element::Object(Object::GHOST),
        Noun::TEXT => Element::Text(Text::Nominal(Nominal::Noun(get_noun(original_element)))),
    }
}
