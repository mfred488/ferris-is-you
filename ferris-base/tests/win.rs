use ferris_base;

mod utils;

#[test]
fn nominal() {
    let start = vec![
        "🦀..........",
        "🚀..........",
        "Ro==Wi......",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::DOWN];
    let end = vec![
        "............",
        "🚀..........",
        "Ro==Wi......",
        "Fe==U ......",
    ];

    let win = utils::assert_evolution(start, inputs, end);
    assert_eq!(win, true);
}

#[test]
fn simultaneous_you_and_win() {
    let start = vec![
        "Fe==U ......",
        "==..........",
        "..Wi🦀......",
        "............",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::LEFT];
    let end = vec![
        "Fe==U ......",
        "==..........",
        "Wi🦀........",
        "............",
    ];

    let win = utils::assert_evolution(start, inputs, end);
    assert_eq!(win, true);
}
