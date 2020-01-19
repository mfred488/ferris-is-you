use super::element::*;

pub enum Rule {
    NounIsAdjectiveRule(NounIsAdjectiveRule),
    NounIsNounRule(NounIsNounRule),
}

#[derive(Debug)]
pub struct NounIsAdjectiveRule {
    pub noun: Noun,
    pub adjective: Adjective,
}

#[derive(Debug)]
pub struct NounIsNounRule {
    pub left: Noun,
    pub right: Noun,
}

pub fn is_rule(el1: &Element, el2: &Element, el3: &Element) -> Option<Rule> {
    match (el1, el2, el3) {
        (
            Element::Text(Text::Noun(noun)),
            Element::Text(Text::IS),
            Element::Text(Text::Adjective(adjective)),
        ) => Some(Rule::NounIsAdjectiveRule(NounIsAdjectiveRule {
            noun: noun.clone(),
            adjective: adjective.clone(),
        })),
        (
            Element::Text(Text::Noun(left)),
            Element::Text(Text::IS),
            Element::Text(Text::Noun(right)),
        ) => Some(Rule::NounIsNounRule(NounIsNounRule {
            left: left.clone(),
            right: right.clone(),
        })),
        _ => None,
    }
}
