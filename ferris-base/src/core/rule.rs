use super::element::*;

pub enum Rule {
    NounIsNominalRule(NounIsNominalRule),
    NounsGroupIsNominalsGroupRule(NounsGroupIsNominalsGroupRule),
    NounOnNounsGroupIsNominalsGroupRule(NounOnNounsGroupIsNominalsGroupRule),
    NounHasNounsRule(NounHasNounsRule),
    QualifiedNounIsNominalsGroupRule(QualifiedNounIsNominalsGroupRule),
}

#[derive(Debug)]
pub enum QualifiedNoun {
    SimpleNoun(Noun),
    NounNearNoun { subject: Noun, near_noun: Noun },
    NounFacingNoun { subject: Noun, facing_noun: Noun },
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

// TODO Make that a QualifiedNounIsNominalsGroupRule
#[derive(Debug)]
pub struct NounOnNounsGroupIsNominalsGroupRule {
    pub subject: Noun,
    pub underlying_nouns: Vec<Noun>,
    pub nominals: Vec<Nominal>,
}

#[derive(Debug)]
pub struct NounHasNounsRule {
    pub subject: Noun,
    pub objects: Vec<Noun>,
}

#[derive(Debug)]
pub struct QualifiedNounIsNominalsGroupRule {
    pub qualified_noun: QualifiedNoun,
    pub nominals: Vec<Nominal>,
}

/**
 * TODO unsupported rules:
 *  - length 5:
 *      - A on/near/facing B has C
 *  - length 6: not supported
 *  - length 7
 *      - A and B has C and D
 *      - A and B on/facing/near C is/has D
 *  - length 8+: not supported
 */

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
            Element::Text(Text::Misc(Misc::AND)),
            Element::Text(Text::Nominal(nominal2)),
        ) => Some(Rule::NounsGroupIsNominalsGroupRule(
            NounsGroupIsNominalsGroupRule {
                nouns: vec![noun.clone()],
                nominals: vec![nominal1.clone(), nominal2.clone()],
            },
        )),
        (
            Element::Text(Text::Nominal(Nominal::Noun(noun1))),
            Element::Text(Text::Misc(Misc::AND)),
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
            Element::Text(Text::Misc(Misc::AND)),
            Element::Text(Text::Nominal(Nominal::Noun(object2))),
        ) => Some(Rule::NounHasNounsRule(NounHasNounsRule {
            subject: subject.clone(),
            objects: vec![object1.clone(), object2.clone()],
        })),
        (
            Element::Text(Text::Nominal(Nominal::Noun(subject))),
            Element::Text(Text::Misc(Misc::ON)),
            Element::Text(Text::Nominal(Nominal::Noun(underlying_noun))),
            Element::Text(Text::Verb(Verb::IS)),
            Element::Text(Text::Nominal(nominal)),
        ) => Some(Rule::NounOnNounsGroupIsNominalsGroupRule(
            NounOnNounsGroupIsNominalsGroupRule {
                subject: subject.clone(),
                underlying_nouns: vec![underlying_noun.clone()],
                nominals: vec![nominal.clone()],
            },
        )),
        (
            Element::Text(Text::Nominal(Nominal::Noun(subject))),
            Element::Text(Text::Misc(Misc::NEAR)),
            Element::Text(Text::Nominal(Nominal::Noun(near_noun))),
            Element::Text(Text::Verb(Verb::IS)),
            Element::Text(Text::Nominal(nominal)),
        ) => Some(Rule::QualifiedNounIsNominalsGroupRule(
            QualifiedNounIsNominalsGroupRule {
                qualified_noun: QualifiedNoun::NounNearNoun {
                    subject: subject.clone(),
                    near_noun: near_noun.clone(),
                },
                nominals: vec![nominal.clone()],
            },
        )),
        (
            Element::Text(Text::Nominal(Nominal::Noun(subject))),
            Element::Text(Text::Misc(Misc::FACING)),
            Element::Text(Text::Nominal(Nominal::Noun(facing_noun))),
            Element::Text(Text::Verb(Verb::IS)),
            Element::Text(Text::Nominal(nominal)),
        ) => Some(Rule::QualifiedNounIsNominalsGroupRule(
            QualifiedNounIsNominalsGroupRule {
                qualified_noun: QualifiedNoun::NounFacingNoun {
                    subject: subject.clone(),
                    facing_noun: facing_noun.clone(),
                },
                nominals: vec![nominal.clone()],
            },
        )),
        _ => None,
    }
}

pub fn is_rule_7(
    el1: &Element,
    el2: &Element,
    el3: &Element,
    el4: &Element,
    el5: &Element,
    el6: &Element,
    el7: &Element,
) -> Option<Rule> {
    match (el1, el2, el3, el4, el5, el6, el7) {
        (
            Element::Text(Text::Nominal(Nominal::Noun(noun))),
            Element::Text(Text::Verb(Verb::IS)),
            Element::Text(Text::Nominal(nominal1)),
            Element::Text(Text::Misc(Misc::AND)),
            Element::Text(Text::Nominal(nominal2)),
            Element::Text(Text::Misc(Misc::AND)),
            Element::Text(Text::Nominal(nominal3)),
        ) => Some(Rule::NounsGroupIsNominalsGroupRule(
            NounsGroupIsNominalsGroupRule {
                nouns: vec![noun.clone()],
                nominals: vec![nominal1.clone(), nominal2.clone(), nominal3.clone()],
            },
        )),
        (
            Element::Text(Text::Nominal(Nominal::Noun(noun1))),
            Element::Text(Text::Misc(Misc::AND)),
            Element::Text(Text::Nominal(Nominal::Noun(noun2))),
            Element::Text(Text::Misc(Misc::AND)),
            Element::Text(Text::Nominal(Nominal::Noun(noun3))),
            Element::Text(Text::Verb(Verb::IS)),
            Element::Text(Text::Nominal(nominal)),
        ) => Some(Rule::NounsGroupIsNominalsGroupRule(
            NounsGroupIsNominalsGroupRule {
                nouns: vec![noun1.clone(), noun2.clone(), noun3.clone()],
                nominals: vec![nominal.clone()],
            },
        )),
        (
            Element::Text(Text::Nominal(Nominal::Noun(subject))),
            Element::Text(Text::Verb(Verb::HAS)),
            Element::Text(Text::Nominal(Nominal::Noun(object1))),
            Element::Text(Text::Misc(Misc::AND)),
            Element::Text(Text::Nominal(Nominal::Noun(object2))),
            Element::Text(Text::Misc(Misc::AND)),
            Element::Text(Text::Nominal(Nominal::Noun(object3))),
        ) => Some(Rule::NounHasNounsRule(NounHasNounsRule {
            subject: subject.clone(),
            objects: vec![object1.clone(), object2.clone(), object3.clone()],
        })),
        (
            Element::Text(Text::Nominal(Nominal::Noun(subject))),
            Element::Text(Text::Misc(Misc::ON)),
            Element::Text(Text::Nominal(Nominal::Noun(underlying_noun1))),
            Element::Text(Text::Misc(Misc::AND)),
            Element::Text(Text::Nominal(Nominal::Noun(underlying_noun2))),
            Element::Text(Text::Verb(Verb::IS)),
            Element::Text(Text::Nominal(nominal)),
        ) => Some(Rule::NounOnNounsGroupIsNominalsGroupRule(
            NounOnNounsGroupIsNominalsGroupRule {
                subject: subject.clone(),
                underlying_nouns: vec![underlying_noun1.clone(), underlying_noun2.clone()],
                nominals: vec![nominal.clone()],
            },
        )),
        (
            Element::Text(Text::Nominal(Nominal::Noun(subject))),
            Element::Text(Text::Misc(Misc::ON)),
            Element::Text(Text::Nominal(Nominal::Noun(underlying_noun))),
            Element::Text(Text::Verb(Verb::IS)),
            Element::Text(Text::Nominal(nominal1)),
            Element::Text(Text::Misc(Misc::AND)),
            Element::Text(Text::Nominal(nominal2)),
        ) => Some(Rule::NounOnNounsGroupIsNominalsGroupRule(
            NounOnNounsGroupIsNominalsGroupRule {
                subject: subject.clone(),
                underlying_nouns: vec![underlying_noun.clone()],
                nominals: vec![nominal1.clone(), nominal2.clone()],
            },
        )),
        _ => None,
    }
}
