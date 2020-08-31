use ferris_base;

mod utils;

#[test]
fn stops_user_move() {
    let start = vec![
        "..🦀........",
        "..🦀🧱......",
        "............",
        "Fe==U Wa==St",
    ];
    let inputs = vec![ferris_base::core::direction::Direction::RIGHT];
    let end = vec![
        "....🦀......",
        "..🦀🧱......",
        "............",
        "Fe==U Wa==St",
    ];

    utils::assert_evolution(start, inputs, end);
}
