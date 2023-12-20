use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::process;

mod solver1;
mod solver2;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args()
        .collect();
    if args.len() != 3 || (args[1] != "1" && args[1] != "2") {
        eprintln!("Usage:\ncargo run <1|2> <input file>");
        process::exit(1);
    }
    let mut file = File::open(args[2].clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let result = match args[1].as_str() {
        "1" => solver1::solve(&contents),
        "2" => solver2::solve(&contents),
        _ => unreachable!(),
    };

    match result {
        None => println!("Invalid format of the calibration document"),
        Some(res) => println!("Result: {}", res),
    }

    Ok(())
}
