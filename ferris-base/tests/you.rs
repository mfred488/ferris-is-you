use ferris_base;

mod utils;

#[test]
fn move_up() {
    let start = vec![
        "............",
        "..🦀........",
        "............",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::UP];
    let end = vec![
        "..🦀........",
        "............",
        "............",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn move_down() {
    let start = vec![
        "............",
        "..🦀........",
        "............",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::DOWN];
    let end = vec![
        "............",
        "............",
        "..🦀........",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn move_right() {
    let start = vec![
        "............",
        "..🦀........",
        "............",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::RIGHT];
    let end = vec![
        "............",
        "....🦀......",
        "............",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn move_left() {
    let start = vec![
        "............",
        "..🦀........",
        "............",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::LEFT];
    let end = vec![
        "............",
        "🦀..........",
        "............",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn only_you_moves() {
    let start = vec![
        "🦀..........",
        "🚀..........",
        "🦀..........",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::RIGHT];
    let end = vec![
        "..🦀........",
        "🚀..........",
        "..🦀........",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}
