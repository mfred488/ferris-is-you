use ferris_is_you;

mod utils;

#[test]
fn tele_only_on_same_element() {
    let start = vec![
        "🦀🚀....🚀..",
        "🚩⭐....⭐..",
        "Ro&&Sr==Te..",
        "Fe&&Fg==U ..",
    ];
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let end = vec![
        "..🚀....🚀🦀",
        "..⭐....⭐🚩",
        "Ro&&Sr==Te..",
        "Fe&&Fg==U ..",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
// Baba is you: ??? extra 3
fn tele_to_random_target() {
    let start = vec![
        "🦀⭐....⭐..",
        "........⭐..",
        "Sr==Te......",
        "Fe==U ......",
    ];
    let start_clone = start.clone();
    let inputs = vec![
        ferris_is_you::core::direction::Direction::RIGHT,
        ferris_is_you::core::direction::Direction::RIGHT,
    ];
    let inputs_clone = inputs.clone();
    let end1 = vec![
        "..⭐....⭐..",
        "........⭐🦀",
        "Sr==Te......",
        "Fe==U ......",
    ];
    let end2 = vec![
        "..⭐....⭐🦀",
        "........⭐..",
        "Sr==Te......",
        "Fe==U ......",
    ];

    utils::seeded_assert_evolution(start, inputs, end1, Some([1; 32]));
    utils::seeded_assert_evolution(start_clone, inputs_clone, end2, Some([2; 32]));
}

#[test]
fn two_tele_items_same_location() {}
