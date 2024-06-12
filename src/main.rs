use std::env;
mod grid;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("File path is required");
        std::process::exit(1);
    }
    match grid::Grid::from_file(&args[1]) {
        Ok(mut grid) => match grid.calculate() {
            Ok(table) => {
                for i in 0..9 {
                    for j in 0..9 {
                        if j == 2 || j == 5 {
                            print!("{} | ", table[i][j]);
                        } else {
                            print!("{} ", table[i][j]);
                        }
                    }
                    println!("\n---------------------");
                }
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
