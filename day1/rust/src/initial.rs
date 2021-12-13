use std::num::ParseIntError;

pub(crate) fn run(inputs: &str) -> Result<usize, String> {
    let numbers: Vec<u32> = inputs.lines()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
        .map_err(|e| format!("Error in parsing number : {}", e))?;

    Ok(numbers.as_slice()
        .windows(2)
        .filter(|&x| x[1] > x[0])
        .count())
}