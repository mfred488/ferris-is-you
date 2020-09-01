use ferris_base;

mod utils;

#[test]
fn facing_noun_is_adjective() {
    let start = vec![
        "🦀..🦀......",
        "🔑..🚩......",
        "............",
        "FeFcFg==U ..",
    ];
    let inputs = vec![
        ferris_base::core::direction::Direction::RIGHT,
        ferris_base::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "🦀....🦀....",
        "🔑..🚩......",
        "............",
        "FeFcFg==U ..",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn facing_noun_is_noun() {
    let start = vec![
        "🦀..🔑......",
        "🦀..🚩......",
        "Fe==U ......",
        "FeFcFg==Ke..",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::RIGHT];
    let end = vec![
        "..🦀🔑......",
        "..🔑🚩......",
        "Fe==U ......",
        "FeFcFg==Ke..",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn facing_takes_orientation_into_account() {
    let start = vec![
        "..🚩........",
        "🦀..........",
        "🦀..🚩......",
        "Fe==U ......",
        "FeFcFg==Ke..",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::RIGHT];
    let end = vec![
        "..🚩........",
        "..🦀........",
        "..🔑🚩......",
        "Fe==U ......",
        "FeFcFg==Ke..",
    ];

    utils::assert_evolution(start, inputs, end);
}
