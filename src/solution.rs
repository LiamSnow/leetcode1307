use std::cmp::Reverse;
use std::fmt::{self, Display};
use std::time::Instant;

pub struct Solution;

impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        println!("\n{}", "=".repeat(20));
        println!("{} = {}", words.join(" + "), result);

        let start = Instant::now();
        let map = Map::new(&words, &result);
        println!("Built map in {:?}", start.elapsed());
        println!("{map}");

        let start = Instant::now();
        let mut eq = Equation::new(map);
        println!("Built equation in {:?}", start.elapsed());

        let start = Instant::now();
        let res = eq.solve();
        println!("Solved equation in {:?}", start.elapsed());

        println!("{res}");
        println!("{}", "=".repeat(20));

        res
    }
}

// ----------------------- Mapping ------------------------------

/// Range of uppercase Ascii characters (constraint #3)
const CHAR_RNG: usize = (b'Z' as usize) - (b'A' as usize) + 1;

#[derive(Clone)]
struct Map([MapTerm; CHAR_RNG]);

#[derive(Clone, Copy, Debug)]
struct MapTerm {
    coeff: i32,
    zeroable: bool,
}

impl Map {
    const fn blank() -> Self {
        Self(
            [MapTerm {
                zeroable: true,
                coeff: 0,
            }; CHAR_RNG],
        )
    }

    fn new(words: &[String], result: &str) -> Self {
        let mut map = Self::blank();

        map.add_word(result, Side::Right);

        for word in words {
            map.add_word(word, Side::Left);
        }

        map
    }

    const fn add_word(&mut self, word: &str, side: Side) {
        let mut coeff = match side {
            Side::Left => -1,
            Side::Right => 1,
        };

        let mut i = word.len();
        while i > 0 {
            i -= 1;
            let index = word.as_bytes()[i] - b'A';
            let term = &mut self.0[index as usize];
            term.coeff += coeff;
            if i == 0 {
                term.zeroable = false;
            }
            coeff *= 10;
        }
    }
}

// ----------------------- Solver ------------------------------

#[derive(Clone, Default)]
struct Equation {
    lhs: Expr,
    rhs: Expr,
}

impl Equation {
    fn new(map: Map) -> Self {
        let mut me = Self::default();

        for (i, term) in map.0.into_iter().enumerate() {
            let new_term = Term {
                coeff: term.coeff.unsigned_abs(),
                var: ((i as u8) + b'A') as char,
                zeroable: term.zeroable,
                value: None,
            };

            if term.coeff > 0 {
                me.lhs.0.push(new_term);
            } else if term.coeff < 0 {
                me.rhs.0.push(new_term);
            }
        }

        me.lhs.0.sort_unstable_by_key(|term| Reverse(term.coeff));
        me.rhs.0.sort_unstable_by_key(|term| Reverse(term.coeff));
        me
    }

    fn solve(&mut self) -> bool {
        self.solve_rec(DigitSet::default())
    }

    fn solve_rec(&mut self, mut set: DigitSet) -> bool {
        let Some((side, idx)) = self.next_free() else {
            return self.is_solved();
        };

        for digit in self.term(side, idx).floor()..=9 {
            if set.is_taken(digit) {
                continue;
            }

            set.take(digit);
            self.term(side, idx).value = Some(digit);

            let least_we_can_do = self.side_mut(side).bound(set, Bound::Min);
            let most_they_can_do = self.side_mut(side.inverse()).bound(set, Bound::Max);

            if least_we_can_do > most_they_can_do {
                self.term(side, idx).value = None;
                break;
            }

            if self.solve_rec(set) {
                self.term(side, idx).value = None;
                return true;
            }

            self.term(side, idx).value = None;
            set.free(digit);
        }

        self.term(side, idx).value = None;
        false
    }

    fn side_mut(&mut self, side: Side) -> &mut Expr {
        match side {
            Side::Left => &mut self.lhs,
            Side::Right => &mut self.rhs,
        }
    }

    fn term(&mut self, side: Side, idx: usize) -> &mut Term {
        &mut self.side_mut(side).0[idx]
    }

    fn next_free(&self) -> Option<(Side, usize)> {
        Some(match (self.lhs.next_free(), self.rhs.next_free()) {
            (Some(l), Some(r)) => {
                if self.lhs.0[l].coeff > self.rhs.0[r].coeff {
                    (Side::Left, l)
                } else {
                    (Side::Right, r)
                }
            }
            (Some(l), None) => (Side::Left, l),
            (None, Some(r)) => (Side::Right, r),
            (None, None) => return None,
        })
    }

    fn is_solved(&self) -> bool {
        self.lhs.sum() == self.rhs.sum()
    }
}

#[derive(Clone, Default)]
struct Expr(Vec<Term>);

enum Bound {
    Min,
    Max,
}

impl Expr {
    fn next_free(&self) -> Option<usize> {
        self.0.iter().position(|term| term.value.is_none())
    }

    fn bound(&self, mut set: DigitSet, dir: Bound) -> u32 {
        let mut res = 0;
        for term in &self.0 {
            let value = match term.value {
                Some(v) => v,
                None => match dir {
                    Bound::Min => set.take_first(term.floor()..=9),
                    Bound::Max => set.take_first((term.floor()..=9).rev()),
                }
                .expect("more terms than free digits"),
            };
            res += value as u32 * term.coeff;
        }
        res
    }

    fn sum(&self) -> u32 {
        self.0
            .iter()
            .map(|term| {
                let value = term
                    .value
                    .expect("equation was checked before being solved");
                term.coeff * (value as u32)
            })
            .sum()
    }
}

#[derive(Clone, Default)]
struct Term {
    coeff: u32,
    var: char,
    zeroable: bool,
    value: Option<u8>,
}

impl Term {
    fn floor(&self) -> u8 {
        if self.zeroable { 0 } else { 1 }
    }
}

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl Side {
    fn inverse(&self) -> Self {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct DigitSet(u16);

impl DigitSet {
    fn is_taken(&self, digit: u8) -> bool {
        (self.0 & (1 << digit)) != 0
    }

    fn take(&mut self, digit: u8) {
        self.0 |= 1 << digit;
    }
    fn free(&mut self, digit: u8) {
        self.0 &= !(1 << digit);
    }

    fn take_first(&mut self, of: impl Iterator<Item = u8>) -> Option<u8> {
        for d in of {
            if !self.is_taken(d) {
                self.take(d);
                return Some(d);
            }
        }
        None
    }
}

// ----------------------- Debug ------------------------------

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let eq = Equation::new(self.clone());
        write!(f, "{eq}")
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.lhs, self.rhs)?;

        writeln!(f)?;

        let mut zeroable = self
            .lhs
            .0
            .iter()
            .chain(self.rhs.0.iter())
            .filter(|term| !term.zeroable)
            .peekable();

        if zeroable.peek().is_some() {
            write!(f, "where ")?;
            for (i, term) in zeroable.enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }

                write!(f, "{}", term.var.to_ascii_lowercase())?;
            }
            write!(f, " != 0")?;
        }

        Ok(())
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, term) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, " + ")?;
            }

            match term.value {
                Some(v) => {
                    write!(f, "{}({})", term.coeff, v)?;
                }
                None => {
                    write!(f, "{} {} ", term.coeff, term.var.to_ascii_lowercase())?;
                }
            }
        }

        Ok(())
    }
}
