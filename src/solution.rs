use std::{
    collections::HashMap,
    fmt::{self, Display},
    time::Instant,
};

pub struct Solution;

struct KeyedEquation(HashMap<char, Variable>);

#[derive(Debug)]
struct Variable {
    zeroable: bool,
    coefficient: i32,
    /// 0-9
    value: u8,
}

impl Variable {
    pub fn new(zeroable: bool, coefficient: i32) -> Self {
        Self {
            zeroable,
            coefficient,
            value: 0,
        }
    }
}

enum Side {
    Left,
    Right,
}

impl KeyedEquation {
    fn new(words: &[String], result: &str) -> Self {
        // constraint #5
        let mut map = Self(HashMap::with_capacity(10));

        map.add_word(result, Side::Right);

        for word in words {
            map.add_word(word, Side::Left);
        }

        debug_assert!(map.0.len() <= 10);

        map
    }

    fn add_word(&mut self, word: &str, side: Side) {
        // println!("+ {word}");
        let mut power = match side {
            Side::Left => -1,
            Side::Right => 1,
        };

        let mut iter = word.chars().rev().peekable();

        while let Some(char) = iter.next() {
            // iterator is reversed, so last loop cannot be zeroed
            let zeroable = iter.peek().is_some();

            match self.0.get_mut(&char) {
                Some(term) => {
                    term.coefficient += power;

                    if !zeroable {
                        // println!("\t= {char} is now not zeroable");
                        term.zeroable = false;
                    }
                }
                None => {
                    // println!("\t+ {char}({zeroable}, {power})");
                    self.0.insert(char, Variable::new(zeroable, power));
                }
            }

            power *= 10;
        }
    }
}

impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        println!("\n{}", "=".repeat(20));
        println!("{} = {}", words.join(" + "), result);

        let start = Instant::now();
        let eq = KeyedEquation::new(&words, &result);
        println!("{eq}");
        println!("Building equation {:?}", start.elapsed());

        let start = Instant::now();
        let mut eq = Equation::new(eq);
        let res = eq.solve();
        println!("Solving took {:?}", start.elapsed());
        println!("{}", "=".repeat(20));

        res
    }
}

struct Equation(Vec<Variable>);

impl Equation {
    fn new(eq: KeyedEquation) -> Self {
        Self(eq.0.into_values().collect())
    }

    fn solve(&mut self) -> bool {
        self.solve_rec(0)
    }

    fn solve_rec(&mut self, pos: usize) -> bool {
        for num in 0..=9 {
            if num == 0 && !self.0[pos].zeroable {
                continue;
            }

            if self.0[0..pos].iter().any(|var| var.value == num) {
                continue;
            }

            self.0[pos].value = num;

            if pos != self.0.len() - 1 {
                if self.solve_rec(pos + 1) {
                    return true;
                }
            } else if self.is_solved() {
                return true;
            }
        }
        false
    }

    fn is_solved(&self) -> bool {
        let mut total = 0;
        for var in &self.0 {
            total += var.coefficient * (var.value as i32);
        }
        total == 0
    }
}

impl Display for KeyedEquation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted = self.0.iter().collect::<Vec<_>>();
        sorted.sort_by_key(|(_, c)| -c.coefficient);

        for (
            i,
            (
                char,
                Variable {
                    coefficient: power, ..
                },
            ),
        ) in sorted.into_iter().enumerate()
        {
            if *power < 0 {
                write!(f, " - ")?;
            } else if i != 0 {
                write!(f, " + ")?;
            }

            write!(f, "{}{}", power.abs(), char)?;
        }
        writeln!(f, " = 0")?;

        let zeroable = self
            .0
            .iter()
            .filter(|(_, term)| !term.zeroable)
            .map(|(char, _)| char.to_string())
            .collect::<Vec<_>>();

        if !zeroable.is_empty() {
            write!(f, "where {} != 0", zeroable.join(", "))?;
        }

        Ok(())
    }
}
