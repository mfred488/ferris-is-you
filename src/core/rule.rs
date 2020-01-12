use super::element::*;

#[derive(Debug)]
pub struct NounIsAdjectiveRule {
    pub noun: Noun,
    pub adjective: Adjective,
}

pub fn is_rule(el1: &Element, el2: &Element, el3: &Element) -> Option<NounIsAdjectiveRule> {
    match (el1, el2, el3) {
        (
            Element::Text(Text::Noun(noun)),
            Element::Text(Text::IS),
            Element::Text(Text::Adjective(adjective)),
        ) => Some(NounIsAdjectiveRule {
            noun: noun.clone(),
            adjective: adjective.clone(),
        }),
        _ => None,
    }
}
