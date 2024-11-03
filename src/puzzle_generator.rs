use std::{collections::HashSet, fs};

/// The directory where the puzzle files are stored.
pub fn puzzle_dir() -> String {
    format!("{}/puzzle", env!("CARGO_MANIFEST_DIR"))
}

/// Possible puzzle generator results.
pub enum GeneratorResult {
    /// The puzzle files were generated successfully and saved to the file system in the puzzle directory.
    PuzzleFilesGenerated,
    /// The source file for the puzzle was not found.
    SourceFileNotFound,
}

/// Generates the puzzle files.
pub fn generate_puzzle() -> GeneratorResult {
    let puzzle_source = match fs::read_to_string(format!("{}/source.txt", puzzle_dir())) {
        Ok(puzzle_source) => puzzle_source,
        Err(_) => return GeneratorResult::SourceFileNotFound,
    };

    let mut chars: Vec<char> = puzzle_source
        .chars()
        .map(|c| c)
        .collect::<HashSet<char>>()
        .iter()
        .map(|&c| c)
        .collect();
    chars.sort();

    fs::write(
        format!("{}/chars.txt", puzzle_dir()),
        format!("{}", chars.iter().collect::<String>()),
    )
    .expect("Unable to write to chars.txt");

    let mut char_indices: Vec<usize> = Vec::new();
    for line in puzzle_source.lines() {
        for char in line.chars() {
            let char_index = chars
                .iter()
                .position(|&c| c == char)
                .expect("Char not found");
            char_indices.push(char_index);
        }
        char_indices.push(0);
    }
    let char_indices_hex: Vec<String> = char_indices.iter().map(|&i| format!("{}", i)).collect();
    fs::write(
        format!("{}/indices.txt", puzzle_dir()),
        format!("{}", char_indices_hex.join(",")),
    )
    .expect("Unable to write to indices.txt");

    GeneratorResult::PuzzleFilesGenerated
}
