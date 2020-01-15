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
// Original game: Lake 2
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
        "....🦀🌊🚩..",
        "Fg==FlFg==Pu",
        "Fe==U Wt==Si",
    ];

    utils::assert_evolution(start, inputs, end);
}
