use ferris_base;

mod utils;

#[test]
// Baba is you: Garden 1
fn noun_is_noun_and_noun() {
    let start = vec![
        "🚀🦀........",
        "..==........",
        "Ro..Ke&&Fg..",
        "Fe==U Fg==U ",
    ];
    let inputs = vec![
        ferris_base::core::direction::Direction::DOWN,
        ferris_base::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "🔑🚩........",
        "....🦀......",
        "Ro==Ke&&Fg..",
        "Fe==U Fg==U ",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn noun_is_noun_and_adjective() {
    let start = vec![
        "🚀....🦀....",
        "......&&....",
        "Ro==U ..Ke..",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::DOWN];
    let end = vec![
        "............",
        "🔑....🦀....",
        "Ro==U &&Ke..",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn noun_is_adjective_and_adjective() {
    let start = vec![
        "🦀🔥........",
        "............",
        "La==Ho......",
        "Fe==U &&Me..",
    ];
    let inputs = vec![
        ferris_base::core::direction::Direction::RIGHT,
        ferris_base::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "..🔥........",
        "............",
        "La==Ho......",
        "Fe==U &&Me..",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn noun_and_noun_is_noun() {
    let start = vec![
        "..🚩🔥🦀🔑🚀",
        "......==....",
        "Ro&&Ke..Fg..",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::DOWN];
    let end = vec![
        "..🚩🔥..🚩🚩",
        "......🦀....",
        "Ro&&Ke==Fg..",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn noun_and_noun_is_adjective() {
    let start = vec![
        "....🔥🦀🔑🚀",
        "......==....",
        "Ro&&Ke==Ho..",
        "Ro&&Ke..Me..",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::DOWN];
    let end = vec![
        "....🔥......",
        "......🦀....",
        "Ro&&Ke==Ho..",
        "Ro&&Ke==Me..",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn noun_has_noun_and_noun() {
    let start = vec![
        "..🦀🌊......",
        "............",
        "WtHaFg&&Fe..",
        "Fe==U Wt==Si",
    ];
    let inputs = vec![
        ferris_base::core::direction::Direction::RIGHT,
        ferris_base::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "....🚩🦀....",
        "............",
        "WtHaFg&&Fe..",
        "Fe==U Wt==Si",
    ];

    utils::assert_evolution(start, inputs, end);
}
