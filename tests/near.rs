use ferris_is_you;

mod utils;

#[test]
fn nominal() {
    let start = vec![
        "..🦀........",
        "....🚩......",
        "🚀🦀........",
        "🚀🦀........",
        "....👽......",
        "FeNrRo==U ..",
        "FeNrFg==Mv..",
        "FeNrET==Sr..",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::UP,
        ferris_is_you::core::direction::Direction::UP,
    ];
    let end = vec![
        "..🦀........",
        "....🚩......",
        "🚀....🦀....",
        "🚀..⭐......",
        "....👽......",
        "FeNrRo==U ..",
        "FeNrFg==Mv..",
        "FeNrET==Sr..",
    ];

    utils::assert_evolution(start, inputs, end);
}
