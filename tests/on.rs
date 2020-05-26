use ferris_is_you;

mod utils;

#[test]
fn nominal() {
    let start = vec![
        "🦀🚀........",
        "🦀🚀........",
        "🦀..........",
        "FeOnRo==Df..",
        "RoOnFe==Df..",
        "Fe==U ......",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "..🚀........",
        "..🚀........",
        "....🦀......",
        "FeOnRo==Df..",
        "RoOnFe==Df..",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Garden extra 1
fn several_conditions() {
    let start = vec![
        "..👽............",
        "🦀🚀............",
        "🦀🚀............",
        "🦀..............",
        "FeOnRo&&ET==Df..",
        "Fe==U ....ET==Mv",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "................",
        "..🚀............",
        "..🚀🦀..........",
        "....🦀..........",
        "FeOnRo&&ET==Df..",
        "Fe==U ....ET==Mv",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: TODO
fn ferris_on_ferris() {}
