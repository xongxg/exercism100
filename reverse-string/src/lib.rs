
/// you can reference vec collect to reduce the tedious steps
/// https://stackoverflow.com/questions/27996430/reversing-a-string-in-rust
pub fn reverse(input: &str) -> String {
    // let mut res = String::new();
    // for ch in input.chars().rev() {
    //     res.push(ch);
    // }

    // res
    let mut result = input.chars().rev().collect::<String>();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tt = "hello";
        let res = reverse(tt);
        // println!("{}", res);
        assert_eq!(res, "olleh");
    }
}
