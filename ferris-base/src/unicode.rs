use super::core::element::*;
use super::core::level::Level;
use std::fs;
use unicode_segmentation::UnicodeSegmentation;

pub fn element_to_unicode(element: Option<&Element>) -> &str {
    match element {
        Some(Element::Object(Object::FERRIS)) => return "🦀",
        Some(Element::Object(Object::ROCKET)) => return "🚀",
        Some(Element::Object(Object::FLAG)) => return "🚩",
        Some(Element::Object(Object::WALL)) => return "🧱",
        Some(Element::Object(Object::WATER)) => return "🌊",
        Some(Element::Object(Object::LAVA)) => return "🔥",
        Some(Element::Object(Object::KEY)) => return "🔑",
        Some(Element::Object(Object::DOOR)) => return "🚪",
        Some(Element::Object(Object::MOON)) => return "🌙",
        Some(Element::Object(Object::STAR)) => return "⭐",
        Some(Element::Object(Object::BAT)) => return "🦇",
        Some(Element::Object(Object::HAND)) => return "🤚",
        Some(Element::Object(Object::FUNGUS)) => return "🍄",
        Some(Element::Object(Object::KEKE)) => return "👽",
        Some(Element::Object(Object::ICE)) => return "❄️ ",
        Some(Element::Object(Object::SKULL)) => return "💀",
        Some(Element::Object(Object::LOVE)) => return "❤️ ",
        Some(Element::Object(Object::BOX)) => return "📦",
        Some(Element::Object(Object::CLIFF)) => return "⛰️ ",
        Some(Element::Object(Object::GHOST)) => return "👻",
        Some(Element::Object(Object::CLOUD)) => return "☁️",
        Some(Element::Object(Object::ME)) => return "🙂",
        Some(Element::Object(Object::FENCE)) => return "🚧",
        Some(Element::Object(Object::STATUE)) => return "🗿",
        Some(Element::Object(Object::ROCK)) => return "💎",
        Some(Element::Object(Object::GRASS)) => return "🌿",
        Some(Element::Object(Object::FLOWER)) => return "🌼",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::FERRIS)))) => return "Fe",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::ROCKET)))) => return "Ro",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::FLAG)))) => return "Fg",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::WALL)))) => return "Wa",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::WATER)))) => return "Wt",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::TEXT)))) => return "Tx",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::LAVA)))) => return "La",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::KEY)))) => return "Ke",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::DOOR)))) => return "Do",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::MOON)))) => return "Mo",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::STAR)))) => return "Sr",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::BAT)))) => return "Ba",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::HAND)))) => return "Hd",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::FUNGUS)))) => return "Fu",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::KEKE)))) => return "ET",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::ICE)))) => return "Ic",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::SKULL)))) => return "Sk",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::LOVE)))) => return "Lv",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::BOX)))) => return "Bx",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::CLIFF)))) => return "Cf",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::GHOST)))) => return "Gh",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::CLOUD)))) => return "Cd",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::ME)))) => return "M€",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::FENCE)))) => return "Fn",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::STATUE)))) => return "Su",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::ROCK)))) => return "Rc",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::GRASS)))) => return "Gr",
        Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::FLOWER)))) => return "Fw",
        Some(Element::Text(Text::Verb(Verb::IS))) => return "==",
        Some(Element::Text(Text::Verb(Verb::HAS))) => return "Ha",
        Some(Element::Text(Text::Misc(Misc::AND))) => return "&&",
        Some(Element::Text(Text::Misc(Misc::ON))) => return "On",
        Some(Element::Text(Text::Misc(Misc::NEAR))) => return "Nr",
        Some(Element::Text(Text::Misc(Misc::FACING))) => return "Fc",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::YOU)))) => return "U ",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::WIN)))) => return "Wi",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::STOP)))) => return "St",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::PUSH)))) => return "Pu",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::SINK)))) => return "Si",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::FLOAT)))) => return "Fl",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::MOVE)))) => return "Mv",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::DEFEAT)))) => return "Df",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::HOT)))) => return "Ho",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::MELT)))) => return "Me",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::SHIFT)))) => return "Sh",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::OPEN)))) => return "Op",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::SHUT)))) => return "Cl",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::WEAK)))) => return "We",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::PULL)))) => return "Pl",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::TELE)))) => return "Te",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::SWAP)))) => return "Sw",
        Some(Element::Text(Text::Nominal(Nominal::Adjective(Adjective::FALL)))) => return "Fa",
        None => return "..",
    };
}

// Kind of reverse function of the function above; try to de-duplicate that, but without dropping the match (that will detect conflicting definitions)
pub fn unicode_to_element(chars: &str) -> Option<Element> {
    match chars {
        "🦀" => Some(Element::Object(Object::FERRIS)),
        "🚀" => Some(Element::Object(Object::ROCKET)),
        "🚩" => Some(Element::Object(Object::FLAG)),
        "🧱" => Some(Element::Object(Object::WALL)),
        "🌊" => Some(Element::Object(Object::WATER)),
        "🔥" => Some(Element::Object(Object::LAVA)),
        "🔑" => Some(Element::Object(Object::KEY)),
        "🚪" => Some(Element::Object(Object::DOOR)),
        "🌙" => Some(Element::Object(Object::MOON)),
        "⭐" => Some(Element::Object(Object::STAR)),
        "🦇" => Some(Element::Object(Object::BAT)),
        "🤚" => Some(Element::Object(Object::HAND)),
        "🍄" => Some(Element::Object(Object::FUNGUS)),
        "👽" => Some(Element::Object(Object::KEKE)),
        "❄️" => Some(Element::Object(Object::ICE)),
        "💀" => Some(Element::Object(Object::SKULL)),
        "❤️" => Some(Element::Object(Object::LOVE)),
        "📦" => Some(Element::Object(Object::BOX)),
        "⛰️" => Some(Element::Object(Object::CLIFF)),
        "👻" => Some(Element::Object(Object::GHOST)),
        "☁️" => Some(Element::Object(Object::CLOUD)),
        "🙂" => Some(Element::Object(Object::ME)),
        "🚧" => Some(Element::Object(Object::FENCE)),
        "🗿" => Some(Element::Object(Object::STATUE)),
        "💎" => Some(Element::Object(Object::ROCK)),
        "🌿" => Some(Element::Object(Object::GRASS)),
        "🌼" => Some(Element::Object(Object::FLOWER)),
        "Fe" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::FERRIS)))),
        "Ro" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::ROCKET)))),
        "Fg" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::FLAG)))),
        "Wa" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::WALL)))),
        "Wt" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::WATER)))),
        "Tx" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::TEXT)))),
        "La" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::LAVA)))),
        "Ke" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::KEY)))),
        "Do" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::DOOR)))),
        "Mo" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::MOON)))),
        "Sr" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::STAR)))),
        "Ba" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::BAT)))),
        "Hd" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::HAND)))),
        "Fu" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::FUNGUS)))),
        "ET" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::KEKE)))),
        "Ic" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::ICE)))),
        "Sk" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::SKULL)))),
        "Lv" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::LOVE)))),
        "Bx" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::BOX)))),
        "Cf" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::CLIFF)))),
        "Gh" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::GHOST)))),
        "Cd" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::CLOUD)))),
        "M€" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::ME)))),
        "Fn" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::FENCE)))),
        "Su" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::STATUE)))),
        "Rc" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::ROCK)))),
        "Gr" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::GRASS)))),
        "Fw" => Some(Element::Text(Text::Nominal(Nominal::Noun(Noun::FLOWER)))),
        "==" => Some(Element::Text(Text::Verb(Verb::IS))),
        "Ha" => Some(Element::Text(Text::Verb(Verb::HAS))),
        "&&" => Some(Element::Text(Text::Misc(Misc::AND))),
        "On" => Some(Element::Text(Text::Misc(Misc::ON))),
        "Nr" => Some(Element::Text(Text::Misc(Misc::NEAR))),
        "Fc" => Some(Element::Text(Text::Misc(Misc::FACING))),
        "U " => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::YOU,
        )))),
        "Wi" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::WIN,
        )))),
        "St" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::STOP,
        )))),
        "Pu" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::PUSH,
        )))),
        "Si" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::SINK,
        )))),
        "Fl" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::FLOAT,
        )))),
        "Mv" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::MOVE,
        )))),
        "Df" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::DEFEAT,
        )))),
        "Ho" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::HOT,
        )))),
        "Me" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::MELT,
        )))),
        "Sh" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::SHIFT,
        )))),
        "Op" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::OPEN,
        )))),
        "Cl" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::SHUT,
        )))),
        "We" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::WEAK,
        )))),
        "Pl" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::PULL,
        )))),
        "Te" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::TELE,
        )))),
        "Sw" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::SWAP,
        )))),
        "Fa" => Some(Element::Text(Text::Nominal(Nominal::Adjective(
            Adjective::FALL,
        )))),
        ".." => None,
        _ => panic!("Unknown character {}", chars),
    }
}

pub fn build_level_from_lines(lines: std::str::Lines, seed: Option<[u8; 32]>) -> Level {
    let mut width = 0;
    let mut height = 0;
    let mut elements_to_add: Vec<(usize, usize, Element)> = Vec::new();

    for line in lines {
        if line.starts_with("#") {
            continue;
        }

        let mut local_width = 0;
        let graphemes = UnicodeSegmentation::graphemes(line, true).collect::<Vec<&str>>();
        let mut previous_grapheme = "";

        for grapheme in graphemes {
            if grapheme.is_ascii() {
                local_width += 1;
                if (local_width % 2) == 0 {
                    if let Some(element) =
                        unicode_to_element(&(previous_grapheme.to_owned() + grapheme))
                    {
                        elements_to_add.push((local_width / 2 - 1, height, element))
                    }
                } else {
                    previous_grapheme = grapheme;
                }
            } else {
                local_width += 2;
                if let Some(element) = unicode_to_element(grapheme) {
                    elements_to_add.push((local_width / 2 - 1, height, element));
                }
            }
        }

        if width > 0 && local_width != width * 2 {
            panic!(
                "The width of the line {} is inconsistent with the one of previous line(s)!",
                &line
            );
        }
        height += 1;
        width = local_width / 2;
    }

    let mut level = Level::new(width, height, seed);
    for (x, y, element) in elements_to_add {
        level.add_element(x, y, element);
    }

    level
}

pub fn build_level_from_file(file_path: String, seed: Option<[u8; 32]>) -> Level {
    let file_content = fs::read_to_string(file_path).expect("Could not open file !");
    let lines = file_content.lines();

    build_level_from_lines(lines, seed)
}

pub fn get_level_lines(level: &Level) -> Vec<String> {
    let mut lines: Vec<String> = Vec::with_capacity(level.height);
    for y in 0..level.height {
        let mut line = String::with_capacity(level.width);
        for x in 0..level.width {
            let elements = level.get_elements(x, y);
            let first_element = elements.get(0);
            line.push_str(element_to_unicode(first_element))
        }
        lines.push(line);
    }

    lines
}
