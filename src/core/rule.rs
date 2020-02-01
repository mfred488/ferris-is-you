use super::element::*;

pub enum Rule {
    NounIsNominalRule(NounIsNominalRule),
    NounsGroupIsNominalsGroupRule(NounsGroupIsNominalsGroupRule),
    NounHasNounsRule(NounHasNounsRule),
}

#[derive(Debug)]
pub struct NounIsNominalRule {
    pub noun: Noun,
    pub nominal: Nominal,
}

#[derive(Debug)]
pub struct NounsGroupIsNominalsGroupRule {
    pub nouns: Vec<Noun>,
    pub nominals: Vec<Nominal>,
}

#[derive(Debug)]
pub struct NounHasNounsRule {
    pub subject: Noun,
    pub objects: Vec<Noun>,
}

pub fn is_rule_3(el1: &Element, el2: &Element, el3: &Element) -> Option<Rule> {
    match (el1, el2, el3) {
        (
            Element::Text(Text::Nominal(Nominal::Noun(noun))),
            Element::Text(Text::Verb(Verb::IS)),
            Element::Text(Text::Nominal(nominal)),
        ) => Some(Rule::NounIsNominalRule(NounIsNominalRule {
            noun: noun.clone(),
            nominal: nominal.clone(),
        })),
        (
            Element::Text(Text::Nominal(Nominal::Noun(subject))),
            Element::Text(Text::Verb(Verb::HAS)),
            Element::Text(Text::Nominal(Nominal::Noun(object))),
        ) => Some(Rule::NounHasNounsRule(NounHasNounsRule {
            subject: subject.clone(),
            objects: vec![object.clone()],
        })),
        _ => None,
    }
}

pub fn is_rule_5(
    el1: &Element,
    el2: &Element,
    el3: &Element,
    el4: &Element,
    el5: &Element,
) -> Option<Rule> {
    match (el1, el2, el3, el4, el5) {
        (
            Element::Text(Text::Nominal(Nominal::Noun(noun))),
            Element::Text(Text::Verb(Verb::IS)),
            Element::Text(Text::Nominal(nominal1)),
            Element::Text(Text::Conjunction(Conjunction::AND)),
            Element::Text(Text::Nominal(nominal2)),
        ) => Some(Rule::NounsGroupIsNominalsGroupRule(
            NounsGroupIsNominalsGroupRule {
                nouns: vec![noun.clone()],
                nominals: vec![nominal1.clone(), nominal2.clone()],
            },
        )),
        (
            Element::Text(Text::Nominal(Nominal::Noun(noun1))),
            Element::Text(Text::Conjunction(Conjunction::AND)),
            Element::Text(Text::Nominal(Nominal::Noun(noun2))),
            Element::Text(Text::Verb(Verb::IS)),
            Element::Text(Text::Nominal(nominal)),
        ) => Some(Rule::NounsGroupIsNominalsGroupRule(
            NounsGroupIsNominalsGroupRule {
                nouns: vec![noun1.clone(), noun2.clone()],
                nominals: vec![nominal.clone()],
            },
        )),
        (
            Element::Text(Text::Nominal(Nominal::Noun(subject))),
            Element::Text(Text::Verb(Verb::HAS)),
            Element::Text(Text::Nominal(Nominal::Noun(object1))),
            Element::Text(Text::Conjunction(Conjunction::AND)),
            Element::Text(Text::Nominal(Nominal::Noun(object2))),
        ) => Some(Rule::NounHasNounsRule(NounHasNounsRule {
            subject: subject.clone(),
            objects: vec![object1.clone(), object2.clone()],
        })),
        _ => None,
    }
}
