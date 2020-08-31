use ferris_base;

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
        ferris_base::core::direction::Direction::RIGHT,
        ferris_base::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "............",
        "....🚀......",
        "......🦀....",
        "Fe==U Ro==Df",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn defeat_does_not_destroy_not_you() {
    let start = vec![
        "..🦀🦀......",
        "....🚀......",
        "............",
        "Fe==MvRo==Df",
    ];
    let inputs = vec![None, None];
    let end = vec![
        "............",
        "....🚀......",
        "..🦀🦀......",
        "Fe==MvRo==Df",
    ];

    utils::assert_evolution_with_pauses(start, inputs, end);
}
