// pub fn brackets_are_balanced(string: &str) -> bool {
//     let mut brackets = Vec::<char>::new();
//
//     for c in string.chars() {
//         match c {
//             '{' | '(' | '[' => brackets.push(c),
//             ')' => {
//                 if brackets.pop() != Some('(') {
//                     return false;
//                 }
//             }
//             ']' => {
//                 if brackets.pop() != Some('[') {
//                     return false;
//                 }
//             }
//             '}' => {
//                 if brackets.pop() != Some('{') {
//                     return false;
//                 }
//             }
//             _ => (),
//         }
//     }
//
//     brackets.is_empty()
// }

use std::sync::mpsc::channel;

#[derive(Debug)]
pub enum ParseErrorType {
    UnexpectedCharacter(char),
    UnexpectedEOF,
}

#[derive(Debug)]
pub struct ParseError {
    error: ParseErrorType,
    pos: usize,
    expected: char,
    open_at: usize,
}

pub enum Token {
    Open(char),
    Close,
    Other,
}

fn classify(c: char) -> Token {
    match c {
        '(' => Token::Open(')'),
        '[' => Token::Open(']'),
        '{' => Token::Open('}'),
        ')' | ']' | '}' => Token::Close,
        _ => Token::Other,
    }
}

pub fn many_brackets(chars: &[char], pos: usize) -> Result<usize, ParseError> {
    let mut pos = pos;

    while let Some(&c) = chars.get(pos) {
        match classify(c) {
            Token::Open(c) => pos = close_pair(chars, pos + 1, c)?,
            Token::Close => return Ok(pos),
            Token::Other => pos += 1,
        }
    }

    Ok(pos)
}

pub fn close_pair(chars: &[char], pos: usize, close: char) -> Result<usize, ParseError> {
    let new_pose = many_brackets(chars, pos)?;
    let err = |e| {
        Err(ParseError {
            error: e,
            pos: new_pose,
            expected: close,
            open_at: pos - 1,
        })
    };

    match chars.get(new_pose) {
        Some(&c) if c == close => Ok(new_pose + 1),
        Some(&c) => err(ParseErrorType::UnexpectedCharacter(c)),
        None => err(ParseErrorType::UnexpectedEOF),
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    // many_brackets(&string.chars().collect::<Vec<char>>(), 0)
    //     .map(|pos| pos == string.len())
    //     .unwrap_or(false)

    let res = many_brackets(&string.chars().collect::<Vec<char>>(), 0);
    res.map(|pos| pos == string.len()).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paired_square_brackets() {
        assert!(brackets_are_balanced("[]"));
    }
    #[test]
    #[ignore]
    fn empty_string() {
        assert!(brackets_are_balanced(""));
    }
    #[test]
    #[ignore]
    fn unpaired_brackets() {
        assert!(!brackets_are_balanced("[["));
    }
    #[test]
    #[ignore]
    fn wrong_ordered_brackets() {
        assert!(!brackets_are_balanced("}{"));
    }
    #[test]
    #[ignore]
    fn wrong_closing_bracket() {
        assert!(!brackets_are_balanced("{]"));
    }
    #[test]
    #[ignore]
    fn paired_with_whitespace() {
        assert!(brackets_are_balanced("{ }"));
    }
    #[test]
    #[ignore]
    fn partially_paired_brackets() {
        assert!(!brackets_are_balanced("{[])"));
    }
    #[test]
    #[ignore]
    fn simple_nested_brackets() {
        assert!(brackets_are_balanced("{[]}"));
    }
    #[test]
    #[ignore]
    fn several_paired_brackets() {
        assert!(brackets_are_balanced("{}[]"));
    }
    #[test]
    #[ignore]
    fn paired_and_nested_brackets() {
        assert!(brackets_are_balanced("([{}({}[])])"));
    }
    #[test]
    #[ignore]
    fn unopened_closing_brackets() {
        assert!(!brackets_are_balanced("{[)][]}"));
    }
    #[test]
    #[ignore]
    fn unpaired_and_nested_brackets() {
        assert!(!brackets_are_balanced("([{])"));
    }
    #[test]
    #[ignore]
    fn paired_and_wrong_nested_brackets() {
        assert!(!brackets_are_balanced("[({]})"));
    }
    #[test]
    #[ignore]
    fn paired_and_wrong_nested_brackets_but_innermost_are_correct() {
        assert!(!brackets_are_balanced("[({}])"));
    }
    #[test]
    #[ignore]
    fn paired_and_incomplete_brackets() {
        assert!(!brackets_are_balanced("{}["));
    }
    #[test]
    #[ignore]
    fn too_many_closing_brackets() {
        assert!(!brackets_are_balanced("[]]"));
    }
    #[test]
    #[ignore]
    fn early_unexpected_brackets() {
        assert!(!brackets_are_balanced(")()"));
    }
    #[test]
    #[ignore]
    fn early_mismatched_brackets() {
        assert!(!brackets_are_balanced("{)()"));
    }
    #[test]
    #[ignore]
    fn math_expression() {
        assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
    }
    #[test]
    #[ignore]
    fn complex_latex_expression() {
        assert!(brackets_are_balanced(
            "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \\end{array}\\right)"
        ));
    }
}
