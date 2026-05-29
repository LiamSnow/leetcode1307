use crate::solution::Solution;

#[test]
fn example_1() {
    assert!(test(&["SEND", "MORE"], "MONEY"))
}

#[test]
fn example_2() {
    assert!(test(&["SIX", "SEVEN", "SEVEN"], "TWENTY"))
}

#[test]
fn example_3() {
    assert!(!test(&["LEET", "CODE"], "POINT"))
}

fn test(words: &[&str], result: &str) -> bool {
    let words: Vec<String> = words.iter().map(|s| s.to_string()).collect();
    Solution::is_solvable(words, result.to_string())
}
