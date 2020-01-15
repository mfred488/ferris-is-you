use ferris_is_you;

mod utils;

#[test]
fn stops_user_move() {
    let start = vec![
        "..🦀........",
        "..🦀🧱......",
        "............",
        "Fe==U Wa==St",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "....🦀......",
        "..🦀🧱......",
        "............",
        "Fe==U Wa==St",
    ];

    utils::assert_evolution(start, inputs, end);
}
