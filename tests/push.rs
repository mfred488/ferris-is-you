use ferris_is_you;

mod utils;

#[test]
fn pushed_by_user() {
    let start = vec![
        "........🦀🚩",
        "..🦀🚩🚩....",
        "............",
        "Fe==U Fg==Pu",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "........🦀🚩",
        "....🦀🚩🚩..",
        "............",
        "Fe==U Fg==Pu",
    ];

    utils::assert_evolution(start, inputs, end);
}

#[test]
fn text_is_implicit_push() {
    let start = vec![
        "............",
        "..🦀Fe==U ..",
        "............",
        "............",
    ];
    let inputs = vec![ferris_is_you::core::direction::Direction::RIGHT];
    let end = vec![
        "............",
        "....🦀Fe==U ",
        "............",
        "............",
    ];

    utils::assert_evolution(start, inputs, end);
}
