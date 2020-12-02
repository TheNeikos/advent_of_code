use nom::{
    character::complete::{alpha1, anychar, char, digit1, space1},
    combinator::map_res,
    IResult,
};
use std::io::BufRead;
use std::ops::RangeInclusive;

struct PasswordPolicy {
    occurences: RangeInclusive<u64>,
    letter: char,
}

impl PasswordPolicy {
    fn check(&self, password: &str) -> bool {
        self.occurences
            .contains(&(password.matches(self.letter).count() as u64))
    }
}

fn range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (input, min) = map_res(digit1, |num: &str| num.parse::<u64>())(input)?;
    let (input, _) = char('-')(input)?;
    let (input, max) = map_res(digit1, |num: &str| num.parse::<u64>())(input)?;

    Ok((input, min..=max))
}

fn password_policy(input: &str) -> IResult<&str, PasswordPolicy> {
    let (input, occurences) = range(input)?;
    let (input, _) = space1(input)?;
    let (input, letter) = anychar(input)?;

    Ok((input, PasswordPolicy { occurences, letter }))
}

fn password_and_policy(input: &str) -> IResult<&str, (&str, PasswordPolicy)> {
    let (input, policy) = password_policy(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space1(input)?;
    let (input, password) = alpha1(input)?;

    Ok((input, (password, policy)))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let lines: Vec<String> = handle.lines().collect::<Result<_, _>>()?;

    let valid_count = lines
        .iter()
        .map(|line| password_and_policy(&line))
        .flatten()
        .map(|(_, (password, policy))| policy.check(password))
        .filter(|b| *b)
        .count();

    println!("There is a total of {} passwords", valid_count);

    Ok(())
}
