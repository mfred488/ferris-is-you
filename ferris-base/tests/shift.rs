use ferris_base;

mod utils;

#[test]
// Baba is you: Space 8
fn shifts_object() {
    let start = vec![
        "🚀🚩🦀......",
        "............",
        "......Fg==Pu",
        "Fe==U Ro==Sh",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::LEFT];
    let end = vec![
        "🚀🦀........",
        "🚩..........",
        "......Fg==Pu",
        "Fe==U Ro==Sh",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Space 8
fn shift_you() {
    let start = vec![
        "🚀🦀........",
        "............",
        "............",
        "Fe==U Ro==Sh",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::LEFT];
    let end = vec![
        "🚀..........",
        "🦀..........",
        "............",
        "Fe==U Ro==Sh",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn changes_orientation() {
    let start = vec![
        "..Ro..🚩....",
        "..==........",
        "🦀Pu🚀......",
        "Fg==Mv......",
        "Fe==U Ro==Sh",
    ];
    let inputs = vec![
        Some(ferris_base::core::direction::Direction::RIGHT),
        None,
        None,
    ];
    let end = vec![
        "..Ro........",
        "..==........",
        "..🦀Pu🚀..🚩",
        "Fg==Mv......",
        "Fe==U Ro==Sh",
    ];

    utils::assert_evolution_with_pauses(start, inputs, end);
}

#[test]
fn shift_against_stop() {
    let start = vec![
        "..🦀🚩🚀....",
        "......🧱....",
        "Wa==StFg==Pu",
        "Fe==U Ro==Sh",
    ];
    let inputs = vec![
        ferris_base::core::direction::Direction::RIGHT,
        ferris_base::core::direction::Direction::RIGHT,
        ferris_base::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "......🚀🦀🚩",
        "......🧱....",
        "Wa==StFg==Pu",
        "Fe==U Ro==Sh",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Forest 4
fn push_then_shift() {
    let start = vec![
        "🚀..........",
        "🚩..........",
        "🦀..........",
        "......Fg==Pu",
        "Fe==U Ro==Sh",
    ];
    let inputs = vec![
        ferris_base::core::direction::Direction::UP,
        ferris_base::core::direction::Direction::UP,
    ];
    let end = vec![
        "🚀..........",
        "🦀..........",
        "🚩..........",
        "......Fg==Pu",
        "Fe==U Ro==Sh",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Forest 7
fn two_shift_objects_same_direction() {
    let start = vec![
        "🦀..........",
        "🚩..........",
        "🚀....Fg==Sh",
        "......Fg==Pu",
        "Fe==U Ro==Sh",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::DOWN];
    let end = vec![
        "............",
        "🦀..........",
        "......Fg==Sh",
        "🚀....Fg==Pu",
        "Fe==U Ro==Sh",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Forest 7
fn two_shift_objects_different_directions() {
    let start = vec![
        "🦀🚩🚀......",
        "............",
        "......Fg==Sh",
        "......Fg==Pu",
        "Fe==U Ro==Sh",
    ];
    let inputs = vec![
        ferris_base::core::direction::Direction::RIGHT,
        ferris_base::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "....🦀..🚀..",
        "............",
        "......Fg==Sh",
        "......Fg==Pu",
        "Fe==U Ro==Sh",
    ];

    utils::assert_evolution(start, inputs, end);
}
