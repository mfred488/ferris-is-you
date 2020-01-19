use ferris_is_you;

mod utils;

#[test]
fn transforms_objects() {
    let start = vec![
        "......Fe....",
        "......==....",
        "..🦀Ro......",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "......Fe....",
        "......==....",
        "....🚀Ro....",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn transforms_into_text() {
    let start = vec![
        "......Fe....",
        "......==....",
        "..🦀Te......",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "......Fe....",
        "......==....",
        "....FeTe....",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}
