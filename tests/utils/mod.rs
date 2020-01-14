use ferris_is_you;

pub fn assert_evolution(
    start: Vec<&str>,
    inputs: Vec<ferris_is_you::core::direction::Direction>,
    end: Vec<&str>,
) {
    let start_one_line = start.join("\n");
    let start_as_lines = start_one_line.lines();
    let mut level = ferris_is_you::unicode::build_level_from_lines(start_as_lines);
    for input in inputs {
        level.next(input);
    }
    assert_eq!(end, ferris_is_you::unicode::get_level_lines(&level));
}
