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

    for &fst_number in &numbers {
        let fst_rem = 2020 - fst_number;
        for &snd_number in numbers.range(..fst_rem) {
            if let Some(thr_number) = fst_rem.checked_sub(snd_number) {
                if numbers.contains(&thr_number) {
                    println!(
                        "Found the three numbers: {} * {} * {} = {}",
                        fst_number,
                        snd_number,
                        thr_number,
                        fst_number * snd_number * thr_number
                    );
                    return Ok(());
                }
            }
        }
    }

    println!("Could not find the number :C");

    Ok(())
}
