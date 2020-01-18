use ferris_is_you;

mod utils;

#[test]
fn defeat_destroys_you() {
    let start = vec![
        "............",
        "..🦀🚀......",
        "..🦀........",
        "Fe==U Ro==Df",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "............",
        "....🚀......",
        "......🦀....",
        "Fe==U Ro==Df",
    ];

    utils::assert_evolution(start, inputs, end);
}
