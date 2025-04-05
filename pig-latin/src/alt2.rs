use regex::Regex;

pub fn translate_word_1(input: &str) -> String {
    let re_vowel = Regex::new(r"^([aeiou]|xr|yt)").unwrap();
    let re_qu = Regex::new(r"^([^aeiou]*qu)(.*)").unwrap();
    let re_y = Regex::new(r"^([^aeiou]+)y(.*)").unwrap();
    let re_consonant = Regex::new(r"(^[^aeiou]+)(.*)").unwrap();


    if re_vowel.is_match(input) {
        format!("{}ay", input)
    } else if let Some(caps) = re_qu.captures(input) {
        format!("{}{}ay", &caps[2], &caps[1])
    } else if let Some(caps) = re_y.captures(input) {
        format!("y{}{}ay", &caps[2], &caps[1])
    } else if let Some(caps) = re_consonant.captures(input) {
        format!("{}{}ay", &caps[2], &caps[1])
    } else {
        input.to_string()
    }
}
