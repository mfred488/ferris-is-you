use ferris_is_you;

mod utils;

#[test]
// Baba is you: Forest A
fn you_and_swap() {
    let start = vec![
        "🦀🚩........",
        "............",
        "............",
        "Fe==U &&Sw..",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "🚩🦀........",
        "............",
        "............",
        "Fe==U &&Sw..",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Forest C
fn push_and_swap() {
    let start = vec![
        "🦀🚩........",
        "............",
        "Fg==Pu&&Sw..",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "🚩🦀........",
        "............",
        "Fg==Pu&&Sw..",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Forest C
fn does_not_change_orientation() {
    let start = vec![
        "🦀🚩Fg..==Mv",
        "............",
        "Fg==Sw......",
        "Fe==U ......",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "....🦀Fg==Mv",
        "Fg..........",
        "🚩==Sw......",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Forest E
fn can_swap_with_stop() {
    let start = vec![
        "🦀..........",
        "🧱..........",
        "Wa==St......",
        "Fe==Mv&&Sw..",
    ];
    let inputs = vec![None];
    let end = vec![
        "🧱..........",
        "🦀..........",
        "Wa==St......",
        "Fe==Mv&&Sw..",
    ];

    utils::assert_evolution_with_pauses(start, inputs, end);
}
