pub fn str_to_vec(input: &str) -> Vec<u32> {
    input.split("\n\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect()
}
