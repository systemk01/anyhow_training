use anyhow::{anyhow, Context, Ok, Result};

fn main() -> Result<()> {
    let text = read_file("src/info.txt")?;
    println!("File-Content : \n{}", text);

    let area = calculate_square(0, 5)?;
    println!("Area =  {}", area);

    Ok(())

}

fn read_file(path:&str) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("failed to read file {}",path))
}

fn calculate_square(x:u32, y:u32) -> Result<u32> {
    if x == 0 || y == 0 {
        return Err(anyhow!("Calculation of Area with zero values not possible!"));
    }
    Ok(x*y)
}