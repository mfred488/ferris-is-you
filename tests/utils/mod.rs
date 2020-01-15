use ferris_is_you;

pub fn assert_evolution_with_pauses(
    start: Vec<&str>,
    inputs: Vec<Option<ferris_is_you::core::direction::Direction>>,
    end: Vec<&str>,
) -> bool {
    let start_one_line = start.join("\n");
    let start_as_lines = start_one_line.lines();
    let mut level = ferris_is_you::unicode::build_level_from_lines(start_as_lines);
    let mut final_state = false;
    for input in inputs {
        final_state = level.next(input);
    }
    assert_eq!(end, ferris_is_you::unicode::get_level_lines(&level));

    final_state
}

pub fn assert_evolution(
    start: Vec<&str>,
    inputs: Vec<ferris_is_you::core::direction::Direction>,
    end: Vec<&str>,
) -> bool {
    let inputs_as_options = inputs.iter().map(|&dir| Some(dir)).collect();

    assert_evolution_with_pauses(start, inputs_as_options, end)
}
