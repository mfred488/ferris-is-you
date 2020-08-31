use ferris_base;

mod utils;

#[test]
// Baba is you: TODO find a level which proves that behaviour
fn weak_shift_order() {
    let start = vec![
        "🦀🚪........",
        "..🦀........",
        "Fe==WeFeHaKe",
        "Fe==U Do==Sh",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::RIGHT];
    let end = vec![
        "..🚪........",
        "..🔑🦀......",
        "Fe==WeFeHaKe",
        "Fe==U Do==Sh",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: TODO find a level which proves that behaviour
fn weak_has_weak() {}

#[test]
// Baba is you: TODO find a level which proves that behaviour
fn weak_on_weak() {}

#[test]
// Baba is you: TODO find a level which proves that behaviour
fn float_on_weak() {}

#[test]
// Baba is you: Forest 8
fn weak_against_stop_and_edge() {
    let start = vec![
        "🦀🧱........🦀",
        "🚩🧱........🚩",
        "Fg==We..Wa==St",
        "Fg&&Fe==U ....",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::RIGHT];
    let end = vec![
        "🦀🧱........🦀",
        "..🧱..........",
        "Fg==We..Wa==St",
        "Fg&&Fe==U ....",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: TODO find a level which proves that behaviour
fn weak_and_move_against_stop_and_edge_no_turnaround() {
    let start = vec![
        "Fe==Mv&&We..",
        "............",
        "....🦀......",
        "..🦀🧱......",
    ];
    let inputs = vec![
        ferris_base::core::direction::Direction::RIGHT,
        ferris_base::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "Fe==Mv&&We..",
        "............",
        "............",
        "....🧱......",
    ];

    utils::assert_evolution(start, inputs, end);
}
