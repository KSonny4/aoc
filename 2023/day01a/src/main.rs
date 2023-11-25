use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let res = include_str!("input.txt")
        .lines()
    println!("{res:?}");

    Ok(())
}