use std::iter::once;

// pub fn build_proverb(list: &[&str]) -> String {
//     let mut proverb: String = list
//         .windows(2)
//         .map(|wind| format!("For want of a {} the {} was lost.\n", wind[0], wind[1]))
//         .collect();
//
//     if !list.is_empty() {
//         proverb.push_str(&format!("And all for the want of a {}.", list[0]));
//     }
//
//     proverb
// }

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(word) => list
            .windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(once(format!("And all for the want of a {}.", word)))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_pieces() {
        let input = &[];
        let output = build_proverb(input);
        let expected = String::new();
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn one_piece() {
        let input = &["nail"];
        let output = build_proverb(input);
        let expected: String = ["And all for the want of a nail."].join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn two_pieces() {
        let input = &["nail", "shoe"];
        let output = build_proverb(input);
        let expected: String = [
            "For want of a nail the shoe was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn three_pieces() {
        let input = &["nail", "shoe", "horse"];
        let output = build_proverb(input);
        let expected: String = [
            "For want of a nail the shoe was lost.",
            "For want of a shoe the horse was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn full_proverb() {
        let input = &[
            "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
        ];
        let output = build_proverb(input);
        let expected: String = [
            "For want of a nail the shoe was lost.",
            "For want of a shoe the horse was lost.",
            "For want of a horse the rider was lost.",
            "For want of a rider the message was lost.",
            "For want of a message the battle was lost.",
            "For want of a battle the kingdom was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn four_pieces_modernized() {
        let input = &["pin", "gun", "soldier", "battle"];
        let output = build_proverb(input);
        let expected: String = [
            "For want of a pin the gun was lost.",
            "For want of a gun the soldier was lost.",
            "For want of a soldier the battle was lost.",
            "And all for the want of a pin.",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
}
