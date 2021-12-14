mod initial;
mod updated;

fn main() -> Result<(), String> {
    let inputs = include_str!("inputs");
    println!("Initial result : {}", initial::run(inputs)?);
    println!("Updated result : {}", updated::run(inputs)?);
    Ok(())
}
