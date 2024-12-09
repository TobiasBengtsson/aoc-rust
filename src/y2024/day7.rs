use itertools::Itertools;
use nonempty::NonEmpty;
use std::{
    str::FromStr,
    sync::{atomic::AtomicU64, Arc},
};

use crate::utils::template::Solution;

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }
}

type IntType = u64;
type AtomicIntType = AtomicU64;

#[derive(Debug)]
struct Equation {
    result: IntType,
    terms: NonEmpty<IntType>,
}

#[derive(Debug, PartialEq)]
enum EquationParseError {
    TooFewColons,
    TooManyColons,
    ResultNotANumber,
    TermNotANumber,
    ZeroTerms,
}

impl FromStr for Equation {
    type Err = EquationParseError;

    fn from_str(value: &str) -> Result<Self, EquationParseError> {
        let mut colon_split = value.split(':');
        let result_part = colon_split.next().unwrap(); // split returns at least one entry
        let terms_part = colon_split.next().ok_or(EquationParseError::TooFewColons)?;

        if colon_split.next().is_some() {
            return Err(EquationParseError::TooManyColons);
        }

        let result = result_part
            .parse::<IntType>()
            .map_err(|_| EquationParseError::ResultNotANumber)?;

        let terms_vec = terms_part
            .trim()
            .split_whitespace()
            .filter_map(|t| {
                if t.is_empty() {
                    None
                } else {
                    Some(
                        t.parse::<IntType>()
                            .map_err(|_| EquationParseError::TermNotANumber),
                    )
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let terms = NonEmpty::from_vec(terms_vec).ok_or(EquationParseError::ZeroTerms)?;

        Ok(Equation { result, terms })
    }
}

enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    /// Applies the operator to the provided elements.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Some(123), Operator::Add.apply(100, 23));
    /// ```
    fn apply(&self, val1: IntType, val2: IntType) -> Option<IntType> {
        Some(match self {
            Operator::Add => val1.checked_add(val2),
            Operator::Mul => val1.checked_mul(val2),
            Operator::Concat => {
                let new_digit_count = if val2 == 0 {
                    1
                } else {
                    (val2).ilog10() as IntType + 1
                };
                let shift_exponent = (10 as IntType).checked_pow(new_digit_count as u32)?;
                let shifted_val1 = val1.checked_mul(shift_exponent)?;
                // println!("{}, {}, {}, {}", val1, *val2, shifted_val1, shifted_val1 + *val2);
                shifted_val1.checked_add(val2)
            }
        }?)
    }
}

impl Sln {
    fn check_possibly_true(eq: &Equation, operators: &Vec<Operator>) -> bool {
        for ops in
            itertools::repeat_n(operators.iter(), eq.terms.len() - 1).multi_cartesian_product()
        {
            let mut result = *eq.terms.first();
            for (term, op) in eq.terms.tail().iter().zip(ops) {
                result = match op.apply(result, *term) {
                    Some(r) => r,
                    None => break,
                }
            }

            if result == eq.result {
                return true;
            }
        }

        false
    }

    fn sln(&self, input: String, operators: &Vec<Operator>) -> String {
        let equations = input
            .lines()
            .filter(|l| *l != "")
            .map(|l| l.parse::<Equation>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let answer = Arc::new(AtomicIntType::new(0));
        rayon::scope(|s| {
            for eq in equations {
                let answer_clone = answer.clone();
                s.spawn(move |_| {
                    if Self::check_possibly_true(&eq, operators) {
                        answer_clone.fetch_add(eq.result, std::sync::atomic::Ordering::Relaxed);
                    }
                });
            }
        });

        answer
            .load(std::sync::atomic::Ordering::Relaxed)
            .to_string()
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        self.sln(input, &vec![Operator::Add, Operator::Mul])
    }

    fn part_2(&self, input: String) -> String {
        self.sln(input, &vec![Operator::Add, Operator::Mul, Operator::Concat])
    }
}

#[cfg(test)]
mod tests {
    use crate::y2024::day7::EquationParseError;

    use super::*;

    #[test]
    fn test_parse_equation_empty_string() {
        assert_eq!(
            "".parse::<Equation>().unwrap_err(),
            EquationParseError::TooFewColons
        );
    }

    #[test]
    fn test_parse_equation_two_results() {
        assert_eq!(
            "123: 234: 45 34".parse::<Equation>().unwrap_err(),
            EquationParseError::TooManyColons
        );
    }

    #[test]
    fn test_parse_equation_zero_terms() {
        assert_eq!(
            "123: ".parse::<Equation>().unwrap_err(),
            EquationParseError::ZeroTerms
        );
    }

    #[test]
    fn test_parse_equation_non_numeric_result() {
        assert_eq!(
            "1f4wa: 123 234".parse::<Equation>().unwrap_err(),
            EquationParseError::ResultNotANumber
        );
    }

    #[test]
    fn test_parse_equation_non_numeric_term() {
        assert_eq!(
            "123: 123 a2bc 234".parse::<Equation>().unwrap_err(),
            EquationParseError::TermNotANumber
        );
    }

    #[test]
    fn test_operator_add() {
        assert_eq!(Some(0), Operator::Add.apply(0, 0));
        assert_eq!(Some(357), Operator::Add.apply(123, 234));
        assert_eq!(None, Operator::Add.apply(IntType::MAX, IntType::MAX));
    }

    #[test]
    fn test_operator_mul() {
        assert_eq!(Some(0), Operator::Mul.apply(0, 0));
        assert_eq!(Some(132), Operator::Mul.apply(12, 11));
        assert_eq!(None, Operator::Mul.apply(IntType::MAX, IntType::MAX));
    }

    #[test]
    fn test_operator_concat() {
        assert_eq!(Some(0), Operator::Concat.apply(0, 0));
        assert_eq!(Some(100), Operator::Concat.apply(10, 0));
        assert_eq!(Some(1010), Operator::Concat.apply(10, 10));
        assert_eq!(Some(123234), Operator::Concat.apply(123, 234));
        assert_eq!(None, Operator::Concat.apply(IntType::MAX, IntType::MAX));
    }

    #[test]
    fn test_sln1() {
        let input = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        assert_eq!("3749", Sln::new().part_1(input.to_string()));
    }

    #[test]
    fn test_sln2() {
        let input = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        assert_eq!("11387", Sln::new().part_2(input.to_string()));
    }
}
