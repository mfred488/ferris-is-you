use ferris_is_you;

mod utils;

#[test]
// Baba is you: Space 6
fn blocked_by_stop() {
    let start = vec![
        "......🦀..🦀..",
        "......🧱🧱🧱..",
        "Wa==St........",
        "Fe==Fa&&U ....",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "........🦀....",
        "......🧱🧱🧱..",
        "Wa==St........",
        "Fe==Fa&&U ..🦀",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn blocked_by_push() {
    let start = vec![
        "......🦀..🦀..",
        "......🚩🚩🚩..",
        "Fg==St........",
        "Fe==Fa&&U ....",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "........🦀....",
        "......🚩🚩🚩..",
        "Fg==St........",
        "Fe==Fa&&U ..🦀",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Cavern 3
fn can_pull_upwards() {
    let start = vec![
        "............",
        "..........🦀",
        "Fg==Pl....🚩",
        "Fe==Fa&&U ..",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::UP];
    let end = vec![
        "..........🦀",
        "..........🚩",
        "Fg==Pl......",
        "Fe==Fa&&U ..",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Space 10
fn through_defeat() {
    let start = vec![
        "🦀💀....🦀..",
        "Sk==Df..🧱💀",
        "Wa==St......",
        "Fe==Fa&&U ..",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "..💀........",
        "Sk==Df..🧱💀",
        "Wa==St......",
        "Fe==Fa&&U 🦀",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Cavern 3
// TODO understand how this works
fn move_then_fall() {}

#[test]
// Baba is you: Space 10
fn push_then_fall() {
    let start = vec![
        "............",
        "............",
        "Fg==Pu....🚩",
        "Fe==Fa&&U 🦀",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::UP];
    let end = vec![
        "............",
        "..........🚩",
        "Fg==Pu......",
        "Fe==Fa&&U 🦀",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Space 11
fn independent_of_float() {
    let start = vec![
        "............",
        "..........🦀",
        "Wa==St....🧱",
        "Fe==Fa&&Fl..",
    ];
    let inputs = vec![None];
    let end = vec![
        "............",
        "..........🦀",
        "Wa==St....🧱",
        "Fe==Fa&&Fl..",
    ];

    utils::assert_evolution_with_pauses(start, inputs, end);
}

#[test]
fn keeps_orientation() {
    let start = vec![
        "..🦀..........",
        "..🧱🧱🧱......",
        "Wa==St........",
        "Fe==U ........",
        "Fe==Fa&&Mv....",
    ];
    let inputs = vec![
        Some(ferris_is_you::core::direction::Direction::RIGHT),
        None,
        None,
    ];
    let end = vec![
        "..............",
        "..🧱🧱🧱......",
        "Wa==St........",
        "Fe==U ........",
        "Fe==Fa&&Mv🦀..",
    ];

    utils::assert_evolution_with_pauses(start, inputs, end);
}
