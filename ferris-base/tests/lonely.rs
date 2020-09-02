use ferris_base;

mod utils;

#[test]
fn nominal() {
    let start = vec![
        "🦀..........",
        "🦀🚩........",
        "............",
        "LoFe==U ....",
    ];
    let inputs = vec![
        ferris_base::core::direction::Direction::RIGHT,
        ferris_base::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "....🦀......",
        "..🚩........",
        "............",
        "LoFe==U ....",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn two_ferris_not_lonely() {
    let start = vec![
        "🦀..........",
        "🦀..........",
        "............",
        "LoFe==U ....",
    ];
    let inputs = vec![
        ferris_base::core::direction::Direction::UP,
        ferris_base::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "🦀..........",
        "............",
        "............",
        "LoFe==U ....",
    ];

    utils::assert_evolution(start, inputs, end);
}
