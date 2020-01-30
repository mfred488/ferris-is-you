use ferris_is_you;

mod utils;

#[test]
fn sink_destroys_non_floating_object() {
    let start = vec![
        "............",
        "..🦀🚩🌊....",
        "......Fg==Pu",
        "Fe==U Wt==Si",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "............",
        "....🦀......",
        "......Fg==Pu",
        "Fe==U Wt==Si",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Lake 2
fn sink_destroys_several_objects() {
    let start = vec![
        "....Fg==🚩🌊",
        "........Pu..",
        "........🦀..",
        "Fe==U Wt==Si",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::UP,
        ferris_is_you::core::direction::Direction::LEFT,
        ferris_is_you::core::direction::Direction::LEFT,
        ferris_is_you::core::direction::Direction::LEFT,
        ferris_is_you::core::direction::Direction::UP,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "....🦀Fg==..",
        "............",
        "............",
        "Fe==U Wt==Si",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn sink_does_not_destroy_floating_object() {
    let start = vec![
        "............",
        "🦀🚩🚩🌊....",
        "Fg==FlFg==Pu",
        "Fe==U Wt==Si",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "............",
        "....🦀🚩🚩..",
        "Fg==FlFg==Pu",
        "Fe==U Wt==Si",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: Island 1
fn float_on_non_floating_win_does_not_win() {
    let start = vec![
        "🦀🚩........",
        "............",
        "Fe==Fl......",
        "Fe==U Fg==Wi",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "..🦀........",
        "............",
        "Fe==Fl......",
        "Fe==U Fg==Wi",
    ];

    let win = utils::assert_evolution(start, inputs, end);
    assert_eq!(win, false);
}

#[test]
// Baba is you: Island 2
fn float_and_sink_does_not_destroy_non_floating_object() {
    let start = vec![
        "............",
        "🦀🚩🚩🌊....",
        "Wt==FlFg==Pu",
        "Fe==U Wt==Si",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "............",
        "....🦀🌊🚩..",
        "Wt==FlFg==Pu",
        "Fe==U Wt==Si",
    ];

    utils::assert_evolution(start, inputs, end);
}
