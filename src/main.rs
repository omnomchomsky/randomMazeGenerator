use std::env;
mod maze;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3{
        eprintln!("Usage: {} <x_dimension> <y_dimension>", args[0]);
        std::process::exit(1)
    }
    let x_dimension: usize = args[1].parse().expect("x_dimension should be an integer");
    let y_dimension: usize = args[2].parse().expect("y_dimension should be an integer");
    let mut maze = maze::Maze::new(x_dimension, y_dimension);
    maze.generate();
    maze.render_maze();
}
