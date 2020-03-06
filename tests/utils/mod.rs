use ferris_is_you;

pub fn seeded_assert_evolution_with_pauses(
    start: Vec<&str>,
    inputs: Vec<Option<ferris_is_you::core::direction::Direction>>,
    end: Vec<&str>,
    seed: Option<[u8; 32]>,
) -> bool {
    let start_one_line = start.join("\n");
    let start_as_lines = start_one_line.lines();
    let mut level = ferris_is_you::unicode::build_level_from_lines(start_as_lines, seed);
    let mut final_state = false;
    for input in inputs {
        final_state = level.next(input);
    }
    assert_eq!(end, ferris_is_you::unicode::get_level_lines(&level));

    final_state
}

pub fn assert_evolution_with_pauses(
    start: Vec<&str>,
    inputs: Vec<Option<ferris_is_you::core::direction::Direction>>,
    end: Vec<&str>,
) -> bool {
    seeded_assert_evolution_with_pauses(start, inputs, end, None)
}

pub fn seeded_assert_evolution(
    start: Vec<&str>,
    inputs: Vec<ferris_is_you::core::direction::Direction>,
    end: Vec<&str>,
    seed: Option<[u8; 32]>,
) -> bool {
    let inputs_as_options = inputs.iter().map(|&dir| Some(dir)).collect();

    seeded_assert_evolution_with_pauses(start, inputs_as_options, end, seed)
}

pub fn assert_evolution(
    start: Vec<&str>,
    inputs: Vec<ferris_is_you::core::direction::Direction>,
    end: Vec<&str>,
) -> bool {
    seeded_assert_evolution(start, inputs, end, None)
}
