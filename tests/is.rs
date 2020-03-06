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
// Baba is you: Island 1
fn cyclic_transformation() {
    let step1 = vec![
        "Fe........🦀",
        "==..........",
        "Ro==Fg......",
        "....==......",
        "....Fe......",
    ];
    let step2 = vec![
        "Fe........🚀",
        "==..........",
        "Ro==Fg......",
        "....==......",
        "....Fe......",
    ];
    let step3 = vec![
        "Fe........🚩",
        "==..........",
        "Ro==Fg......",
        "....==......",
        "....Fe......",
    ];
    let inputs = vec![None];

    utils::assert_evolution_with_pauses(step1.clone(), inputs.clone(), step2.clone());
    utils::assert_evolution_with_pauses(step2.clone(), inputs.clone(), step3.clone());
    utils::assert_evolution_with_pauses(step3.clone(), inputs.clone(), step1.clone());
}

#[test]
fn transforms_into_text() {
    let start = vec![
        "......Fe....",
        "......==....",
        "..🦀Tx......",
        "Fe==U ......",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "......Fe....",
        "......==....",
        "....FeTx....",
        "Fe==U ......",
    ];

    utils::assert_evolution(start, inputs, end);
}
