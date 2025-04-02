use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// solve1:
/// https://exercism.org/tracks/rust/exercises/alphametics/solutions/HelloWorldPlusPlus
///
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let leading = input
        .trim()
        .split(['=', '+'])
        .filter(|s| !s.is_empty())
        .map(|c| c.trim().chars().next().unwrap())
        .collect::<HashSet<_>>();

    let letters = input
        .trim()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<HashSet<_>>();

    for hm in (0..10)
        .permutations(letters.len())
        .map(|p| {
            letters
                .iter()
                .sorted()
                .zip(p)
                .collect::<HashMap<&char, u8>>()
        })
        .filter(|hm| leading.iter().all(|c| *hm.get(c).unwrap() > 0))
    {
        let res = calc(input.split("==").last()?.trim(), &hm);
        let adds = input
            .trim()
            .split("==")
            .next()?
            .trim()
            .split('+')
            .map(|s| calc(s.trim(), &hm))
            .sum::<u64>();

        if adds == res {
            return Some(
                hm.iter()
                    .map(|(&&c, &i)| (c, i))
                    .collect::<HashMap<char, u8>>(),
            );
        }
    }

    None
}

fn calc(s: &str, hm: &HashMap<&char, u8>) -> u64 {
    // s.trim()
    //     .chars()
    //     .rev()
    //     .enumerate()
    //     .map(|(i, c)| *hm.get(&c).unwrap() as u64 * 10u64.pow(i as u32))
    //     .sum::<u64>()

    s.trim()
        .chars()
        .rev()
        .enumerate()
        .fold(0u64, |acc, (i, c)| {
            acc + *hm.get(&c).unwrap() as u64 * 10u64.pow(i as u32)
        })
}

///
/// solve 2:
/// https://exercism.org/tracks/rust/exercises/alphametics/solutions/887
///
///
// pub fn solve_1(input: &str) -> Option<HashMap<char, u8>> {
//     let letters = input
//         .trim()
//         .chars()
//         .filter(|c| c.is_alphabetic())
//         .sorted()
//         .collect::<HashSet<_>>();
//
//     let equations = input.trim().split("==").collect::<Vec<_>>();
//     let left = equations[0]
//         .split('+')
//         .map(|s| s.trim().chars().collect())
//         .collect::<Vec<_>>();
//
//     let right = equations[1].chars().collect::<Vec<_>>();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_with_three_letters() {
        let answer = solve("I + BB == ILL");
        let expected = [('I', 1), ('B', 9), ('L', 0)].into_iter().collect();
        assert_eq!(answer, Some(expected));
    }
}
