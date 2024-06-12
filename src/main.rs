use std::env;
mod grid;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("File path is required");
        std::process::exit(1);
    }
    match grid::Grid::from_file(&args[1]) {
        Ok(mut grid) => {
            println!("Source grid:\n{}", grid);
            match grid.calculate() {
                Ok(()) => {
                    println!("Solved grid:\n{}", grid);
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
