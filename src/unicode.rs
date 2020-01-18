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
        Some(Element::Text(Text::Noun(Noun::FERRIS))) => return "Fe",
        Some(Element::Text(Text::Noun(Noun::ROCKET))) => return "Ro",
        Some(Element::Text(Text::Noun(Noun::FLAG))) => return "Fg",
        Some(Element::Text(Text::Noun(Noun::WALL))) => return "Wa",
        Some(Element::Text(Text::Noun(Noun::WATER))) => return "Wt",
        Some(Element::Text(Text::Noun(Noun::TEXT))) => return "Te",
        Some(Element::Text(Text::IS)) => return "==",
        Some(Element::Text(Text::Adjective(Adjective::YOU))) => return "U ",
        Some(Element::Text(Text::Adjective(Adjective::WIN))) => return "Wi",
        Some(Element::Text(Text::Adjective(Adjective::STOP))) => return "St",
        Some(Element::Text(Text::Adjective(Adjective::PUSH))) => return "Pu",
        Some(Element::Text(Text::Adjective(Adjective::SINK))) => return "Si",
        Some(Element::Text(Text::Adjective(Adjective::FLOAT))) => return "Fl",
        Some(Element::Text(Text::Adjective(Adjective::MOVE))) => return "Mv",
        Some(Element::Text(Text::Adjective(Adjective::DEFEAT))) => return "Df",
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
        "Fe" => Some(Element::Text(Text::Noun(Noun::FERRIS))),
        "Ro" => Some(Element::Text(Text::Noun(Noun::ROCKET))),
        "Fg" => Some(Element::Text(Text::Noun(Noun::FLAG))),
        "Wa" => Some(Element::Text(Text::Noun(Noun::WALL))),
        "Wt" => Some(Element::Text(Text::Noun(Noun::WATER))),
        "Te" => Some(Element::Text(Text::Noun(Noun::TEXT))),
        "==" => Some(Element::Text(Text::IS)),
        "U " => Some(Element::Text(Text::Adjective(Adjective::YOU))),
        "Wi" => Some(Element::Text(Text::Adjective(Adjective::WIN))),
        "St" => Some(Element::Text(Text::Adjective(Adjective::STOP))),
        "Pu" => Some(Element::Text(Text::Adjective(Adjective::PUSH))),
        "Si" => Some(Element::Text(Text::Adjective(Adjective::SINK))),
        "Fl" => Some(Element::Text(Text::Adjective(Adjective::FLOAT))),
        "Mv" => Some(Element::Text(Text::Adjective(Adjective::MOVE))),
        "Df" => Some(Element::Text(Text::Adjective(Adjective::DEFEAT))),
        ".." => None,
        _ => panic!("Unknown character {}", chars),
    }
}

pub fn build_level_from_lines(lines: std::str::Lines) -> Level {
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

    let mut level = Level::new(width, height);
    for (x, y, element) in elements_to_add {
        level.add_element(x, y, element);
    }

    level
}

pub fn build_level_from_file(file_path: String) -> Level {
    let file_content = fs::read_to_string(file_path).expect("Could not open file !");
    let lines = file_content.lines();

    build_level_from_lines(lines)
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
