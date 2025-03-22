const TEXT: &str = "
1 green bottle2 hanging on the wall,
1 green bottle2 hanging on the wall,
And if one green bottle should accidentally fall,
There'll be 3 green bottle4 hanging on the wall.
";

const UPPERS: [&str; 11] = [
    "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

const LOWER: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut text = String::new();
    for i in 0..take_down {
        let count = start_bottles - i;

        let v1 = UPPERS[count as usize];
        let v2 = if count == 1 { "" } else { "s" };
        let v3 = LOWER[(count - 1) as usize];
        let v4 = if count - 1 == 1 { "" } else { "s" };

        text += &TEXT
            .replace("1", v1)
            .replace("2", v2)
            .replace("3", v3)
            .replace("4", v4);
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_generic_verse() {
        assert_eq!(
            recite(10, 1).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    #[ignore]
    fn last_generic_verse() {
        assert_eq!(
            recite(3, 1).trim(),
            concat!(
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    #[ignore]
    fn verse_with_2_bottles() {
        assert_eq!(
            recite(2, 1).trim(),
            concat!(
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.",
            )
        );
    }
    #[test]
    #[ignore]
    fn verse_with_1_bottle() {
        assert_eq!(
            recite(1, 1).trim(),
            concat!(
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    #[ignore]
    fn first_two_verses() {
        assert_eq!(
            recite(10, 2).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.\n",
                "\n",
                "Nine green bottles hanging on the wall,\n",
                "Nine green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be eight green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    #[ignore]
    fn last_three_verses() {
        assert_eq!(
            recite(3, 3).trim(),
            concat!(
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.\n",
                "\n",
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.\n",
                "\n",
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    #[ignore]
    fn all_verses() {
        assert_eq!(
            recite(10, 10).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.\n",
                "\n",
                "Nine green bottles hanging on the wall,\n",
                "Nine green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be eight green bottles hanging on the wall.\n",
                "\n",
                "Eight green bottles hanging on the wall,\n",
                "Eight green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be seven green bottles hanging on the wall.\n",
                "\n",
                "Seven green bottles hanging on the wall,\n",
                "Seven green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be six green bottles hanging on the wall.\n",
                "\n",
                "Six green bottles hanging on the wall,\n",
                "Six green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be five green bottles hanging on the wall.\n",
                "\n",
                "Five green bottles hanging on the wall,\n",
                "Five green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be four green bottles hanging on the wall.\n",
                "\n",
                "Four green bottles hanging on the wall,\n",
                "Four green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be three green bottles hanging on the wall.\n",
                "\n",
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.\n",
                "\n",
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.\n",
                "\n",
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
}
