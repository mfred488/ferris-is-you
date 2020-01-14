use ferris_is_you;

mod utils;

#[test]
fn only_you_moves() {
    let start = vec![
        "🦀..........",
        "🚀..........",
        "🦀..........",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "..🦀........",
        "🚀..........",
        "..🦀........",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}
