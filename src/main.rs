mod puzzle_generator;
mod puzzle_solver;

use puzzle_generator::{generate_puzzle, GeneratorResult};
use puzzle_solver::solve_puzzle;

fn main() {
    match generate_puzzle() {
        GeneratorResult::PuzzleFilesGenerated => {
            println!("Puzzle files generated successfully.");
        }
        GeneratorResult::SourceFileNotFound => {
            println!("It seems you have to find the solution yourself. Good luck!");
        }
    };

    solve_puzzle();
}
