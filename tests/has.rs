use ferris_is_you;

mod utils;

#[test]
fn appears_after_open() {
    let start = vec![
        "..🔑🚪......",
        "............",
        "Ke==OpKeHaFe",
        "Ke==U Do==Cl",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "....🦀......",
        "............",
        "Ke==OpKeHaFe",
        "Ke==U Do==Cl",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn appears_after_melt() {
    let start = vec![
        "..🦀🔥......",
        "Ke==U ......",
        "Fe==MeFeHaKe",
        "Fe==U La==Ho",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "....🔥🔑....",
        "Ke==U ......",
        "Fe==MeFeHaKe",
        "Fe==U La==Ho",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn appears_after_sink() {
    let start = vec![
        "..🦀🌊......",
        "............",
        "WtHaFgFg==Pu",
        "Fe==U Wt==Si",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "....🚩......",
        "............",
        "WtHaFgFg==Pu",
        "Fe==U Wt==Si",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn appears_after_defeat() {
    let start = vec![
        "..🦀🌊......",
        "............",
        "FeHaFgFg==Sh",
        "Fe==U Wt==Df",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "....🚩🌊....",
        "............",
        "FeHaFgFg==Sh",
        "Fe==U Wt==Df",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn appears_after_weak() {
    let start = vec![
        "..🦀🌊......",
        "............",
        "WtHaFg......",
        "Fe==U Wt==We",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "....🚩🦀....",
        "............",
        "WtHaFg......",
        "Fe==U Wt==We",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn appears_with_ancestors_orientation() {
    let start = vec![
        "..🦀🔥......",
        "Ke==Mv......",
        "Fe==MeFeHaKe",
        "Fe==U La==Ho",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::UP,
    ];
    let end = vec![
        "....🔥🔑....",
        "Ke==Mv......",
        "Fe==MeFeHaKe",
        "Fe==U La==Ho",
    ];

    utils::assert_evolution(start, inputs, end);
}
