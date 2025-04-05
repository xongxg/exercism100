pub fn translate(input: &str) -> String {
    input
        // .trim()
        .split_ascii_whitespace()
        .map(|word| translate_word(word))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn translate_word(input: &str) -> String {
    let n = consonant_chars(input);
    let (consonants, other) = input.split_at(n);
    format!("{}{}ay", other, consonants)
}

fn consonant_chars(input: &str) -> usize {
    if input.is_empty()
        || input.starts_with('o')
        || input.starts_with('e')
        || input.starts_with('i')
        || input.starts_with('u')
        || input.starts_with("a")
        || input.starts_with("xr")
        || input.starts_with("yt")
    {
        0
    } else if input.starts_with("qu") {
        2
    } else if input.chars().nth(1) == Some('y') {
        1
    } else {
        1 + consonant_chars(&input[1..])
    }
}
