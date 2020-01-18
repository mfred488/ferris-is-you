use ferris_is_you;

mod utils;

#[test]
fn hot_destroys_melt() {
    let start = vec![
        "............",
        "..🦀🔥......",
        "Fe==Me......",
        "Fe==U La==Ho",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "............",
        "....🔥......",
        "Fe==Me......",
        "Fe==U La==Ho",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn hot_does_not_destroy_not_melt() {
    let start = vec![
        "............",
        "..🦀🔥......",
        "............",
        "Fe==U La==Ho",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "............",
        "....🔥🦀....",
        "............",
        "Fe==U La==Ho",
    ];

    utils::assert_evolution(start, inputs, end);
}
