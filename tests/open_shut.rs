use ferris_is_you;

mod utils;

#[test]
fn open_destroys_shut() {
    let start = vec![
        "..🔑........",
        "..🔑🚪......",
        "Ke==Op......",
        "Ke==U Do==Cl",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "....🔑......",
        "............",
        "Ke==Op......",
        "Ke==U Do==Cl",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn open_does_not_destroy_not_shut() {
    let start = vec![
        "..🔑........",
        "..🔑🚪......",
        "Ke==Op......",
        "Ke==U ......",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "......🔑....",
        "....🚪🔑....",
        "Ke==Op......",
        "Ke==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn both_open_and_shut() {
    let start = vec![
        "....Ke....🔑",
        "....==......",
        "🦀Op........",
        "Fe==U Ke==Cl",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "....Ke......",
        "....==......",
        "..🦀Op......",
        "Fe==U Ke==Cl",
    ];

    utils::assert_evolution(start, inputs, end);
}
