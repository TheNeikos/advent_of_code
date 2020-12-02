use std::collections::BTreeSet;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let numbers: BTreeSet<u64> = handle
        .lines()
        .flatten()
        .map(|line| str::parse(&line))
        .collect::<Result<_, _>>()?;

    for number in &numbers {
        if numbers.contains(&(2020 - *number)) {
            println!(
                "The number you're looking for is: {} * {} = {}",
                2020 - *number,
                number,
                (2020 - *number) * *number
            );
            return Ok(());
        }
    }

    println!("Could not find the number :C");

    Ok(())
}
