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
        let first =
            password.chars().nth(*self.occurences.start() as usize - 1) == Some(self.letter);
        let second = password.chars().nth(*self.occurences.end() as usize - 1) == Some(self.letter);

        first ^ second
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

    let passwords_policies: Vec<_> = lines
        .iter()
        .map(|line| password_and_policy(&line))
        .collect::<Result<_, _>>()
        .map_err(|input| format!("Invalid line: {}", input))?;

    let valid_count = passwords_policies
        .iter()
        .map(|(_, (password, policy))| policy.check(password))
        .filter(|b| *b)
        .count();

    println!("Check has given us {} valid passwords", valid_count);

    Ok(())
}
