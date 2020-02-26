use ferris_is_you;

mod utils;

#[test]
fn chained_pull() {
    let start = vec![
        "🚩🚩🦀......",
        "............",
        "............",
        "Fe==U Fg==Pl",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "..🚩🚩🦀....",
        "............",
        "............",
        "Fe==U Fg==Pl",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn pulls_only_in_move_direction() {
    let start = vec![
        "............",
        "..🚩🦀......",
        "............",
        "Fe==U Fg==Pl",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::UP];
    let end = vec![
        "....🦀......",
        "..🚩........",
        "............",
        "Fe==U Fg==Pl",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Cavern 3
fn cannot_push_pull() {
    let start = vec![
        "..🚩🦀......",
        "............",
        "............",
        "Fe==U Fg==Pl",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::LEFT];
    let end = vec![
        "..🚩🦀......",
        "............",
        "............",
        "Fe==U Fg==Pl",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Cavern 3
fn can_push_pull_if_explicit() {
    let start = vec![
        "..🚩🦀......",
        "............",
        "......Fg==Pu",
        "Fe==U Fg==Pl",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::LEFT];
    let end = vec![
        "🚩🦀........",
        "............",
        "......Fg==Pu",
        "Fe==U Fg==Pl",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Fall 9
fn pulled_by_move() {
    let start = vec![
        "..🚩........",
        "..🦀........",
        "......Fe==Mv",
        "......Fg==Pl",
    ];
    let inputs = vec![None, None, None];
    let end = vec![
        "............",
        "............",
        "..🚩..Fe==Mv",
        "..🦀..Fg==Pl",
    ];

    utils::assert_evolution_with_pauses(start, inputs, end);
}

#[test]
// Baba is you: TODO find a level which proves that behaviour
fn pulled_by_shift() {
    let start = vec![
        "..🚩........",
        "🦀🚀..Fe==U ",
        "......Ro==Sh",
        "......Fg==Pl",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "............",
        "..🚀..Fe==U ",
        "..🚩🦀Ro==Sh",
        "......Fg==Pl",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: TODO find a level which proves that behaviour
// TODO What happens if we go down ?
fn you_not_pulled_by_move() {
    let start = vec![
        "..🦀........",
        "..🚩..Fe==U ",
        "......Fe==Pl",
        "......Fg==Mv",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "....🦀......",
        "......Fe==U ",
        "..🚩..Fe==Pl",
        "......Fg==Mv",
    ];

    utils::assert_evolution(start, inputs, end);
}
