use ferris_is_you;

mod utils;

#[test]
fn moves_down() {
    let start = vec![
        "............",
        "..🦀........",
        "............",
        "Fe==Mv......",
    ];
    let inputs = vec![None];
    let end = vec![
        "............",
        "............",
        "..🦀........",
        "Fe==Mv......",
    ];

    utils::assert_evolution_with_pauses(start, inputs, end);
}

#[test]
fn half_turn() {
    let start = vec![
        "............",
        "............",
        "............",
        "Fe==Mv🦀....",
    ];
    let inputs = vec![None];
    let end = vec![
        "............",
        "............",
        "......🦀....",
        "Fe==Mv......",
    ];

    utils::assert_evolution_with_pauses(start, inputs, end);
}

#[test]
// Baba is you: ruins 8
fn double_move_with_input() {
    let start = vec![
        "............",
        "..🦀........",
        "Fe==U ......",
        "Fe==Mv......",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "............",
        "......🦀....",
        "Fe==U ......",
        "Fe==Mv......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: ruins 8
fn input_move_evaluation_order() {
    let start = vec![
        "Ro==Mv🚀....",
        "..🦀🚩......",
        "............",
        "Fe==U Fg==Pu",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "Ro==Mv......",
        "....🦀🚀....",
        "......🚩....",
        "Fe==U Fg==Pu",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: ruins 8
fn push_move_evaluation_order() {
    let start = vec![
        "Ro==Mv......",
        "......🚀....",
        "..🦀🚩......",
        "Fe==U Fg==Pu",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "Ro==Mv🚀....",
        "............",
        "....🦀🚩....",
        "Fe==U Fg==Pu",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: ruins extra 1
fn push_then_move() {
    let start = vec![
        "🦀🚩........",
        "............",
        "Fg==Mv......",
        "Fe==U Fg==Pu",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "..🦀..🚩....",
        "............",
        "Fg==Mv......",
        "Fe==U Fg==Pu",
    ];

    utils::assert_evolution(start, inputs, end);
}
