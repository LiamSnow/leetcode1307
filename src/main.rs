fn main() {
    // ["SEND", "MORE"] = "MONEY"
    // Map 'S'-> 9, 'E'->5, 'N'->6, 'D'->7, 'M'->1, 'O'->0, 'R'->8, 'Y'->'2'
    // 1000s + 91e + d + 10r = 9000m + 900o + 90n + y

    let chars = vec![
        CharInfo {
            zeroable: true,
            power: 90,
        },
        CharInfo {
            zeroable: false,
            power: 1,
        },
        CharInfo {
            zeroable: true,
            power: 900,
        },
        CharInfo {
            zeroable: false,
            power: -91,
        },
        CharInfo {
            zeroable: false,
            power: -1,
        },
        CharInfo {
            zeroable: true,
            power: -1000,
        },
        CharInfo {
            zeroable: true,
            power: -10,
        },
        CharInfo {
            zeroable: true,
            power: 9000,
        },
    ];

    let values = vec![6, 2, 0, 5, 7, 9, 8, 1];

    assert!(test(&chars, &values));

    assert!(Solution::is_solvable(
        vec!["SEND".to_string(), "MORE".to_string()],
        "MONEY".to_string()
    ));

    /*

    "SEND", "MORE" = "MONEY"
    Map 'S'-> 9, 'E'->5, 'N'->6, 'D'->7, 'M'->1, 'O'->0, 'R'->8, 'Y'->'2'

    Such that: "SEND" + "MORE" = "MONEY" ,  9567 + 1085 = 10652

            */

    assert!(Solution::is_solvable(
        vec!["SIX".to_string(), "SEVEN".to_string(), "SEVEN".to_string()],
        "TWENTY".to_string()
    ));

    assert!(!Solution::is_solvable(
        vec!["LEET".to_string(), "CODE".to_string()],
        "POINT".to_string()
    ));
}

struct Solution;

use std::collections::HashMap;

#[derive(Debug)]
struct CharInfo {
    zeroable: bool,
    /// negative means left side, positive is right side
    power: i32,
}

impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        // words[i][0], result[0] != 0

        // maps char -> Info
        let mut chars: HashMap<char, CharInfo> = HashMap::with_capacity(10);

        // fill up from `result`
        let mut first = true;
        let mut digit_val = 1;
        for char in result.chars().rev() {
            match chars.get_mut(&char) {
                Some(info) => {
                    info.power += digit_val;
                }
                None => {
                    chars.insert(
                        char,
                        CharInfo {
                            zeroable: !first,
                            power: digit_val,
                        },
                    );
                }
            }
            first = false;
            digit_val *= 10;
        }

        // file up from `words`
        for word in words {
            let mut first = true;
            let mut digit_val = 1;
            for char in word.chars().rev() {
                match chars.get_mut(&char) {
                    Some(info) => {
                        if first {
                            info.zeroable = false;
                        }
                        info.power -= digit_val;
                    }
                    None => {
                        chars.insert(
                            char,
                            CharInfo {
                                zeroable: !first,
                                power: -digit_val,
                            },
                        );
                    }
                }
                first = false;
                digit_val *= 10;
            }
        }

        // quick check
        if chars.len() > 10 {
            return false;
        }

        let chars: Vec<CharInfo> = chars.into_values().collect();
        let mut values: Vec<u8> = vec![0; chars.len()];

        rec(&chars, &mut values, 0)
    }
}

fn rec(chars: &[CharInfo], values: &mut [u8], pos: usize) -> bool {
    for num in 0..=9 {
        if num == 0 && !chars[pos].zeroable {
            continue;
        }

        if values[0..pos].contains(&num) {
            continue;
        }

        values[pos] = num;

        if pos != chars.len() - 1 {
            if rec(chars, values, pos + 1) {
                return true;
            }
        } else if test(chars, values) {
            return true;
        }
    }
    false
}

fn test(chars: &[CharInfo], values: &[u8]) -> bool {
    let mut total = 0;
    assert_eq!(chars.len(), values.len());
    for i in 0..chars.len() {
        total += chars[i].power * values[i] as i32;
    }
    total == 0
}

/*

["SEND", "MORE"] = "MONEY"
Map 'S'-> 9, 'E'->5, 'N'->6, 'D'->7, 'M'->1, 'O'->0, 'R'->8, 'Y'->'2'
Such that: "SEND" + "MORE" = "MONEY" ,  9567 + 1085 = 10652

We can explain as:
1000s + 100e + 10n + d + 1000m + 100o + 10r + e = 10000m + 1000o + 100n + 10e + y
where M != 0, S != 0

Simplify:
1000s + 91e + d + 10r = 9000m + 900o + 90n + y
where M != 0, S != 0

1000(9) + 91(5) + 7 + 10(8)

10^10





*/
