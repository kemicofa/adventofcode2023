pub fn split_and_clean_input_into_lines(input: &str) -> Vec<&str> {
    input
        .trim()
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
}
