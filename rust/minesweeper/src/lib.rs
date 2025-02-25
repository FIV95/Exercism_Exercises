pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Define all possible neighbor directions (Up, Down, Left, Right, and Diagonals)
    const NEIGHBORS: [(isize, isize); 8] = [
        (-1, 0), (-1, -1), (0, -1), (1, -1), // Up, Up-Left, Left, Down-Left
        (1, 0), (1, 1), (0, 1), (-1, 1),     // Down, Down-Right, Right, Up-Right
    ];

    let number_of_rows = minefield.len(); // Get total number of rows in the input
    if number_of_rows == 0 {              // If the board has no rows, return an empty result
        return Vec::new();
    }

    let number_of_columns = minefield[0].len(); // Get number of columns from the first row
    if number_of_columns == 0 {                 // If there are no columns, return a single empty string
        return vec!["".to_string()];
    }

    // Convert the board from a `&str` slice into a `Vec<u8>` for easier indexing
    let byte_board: Vec<&[u8]> = minefield.iter().map(|line| line.as_bytes()).collect();
    let mut final_board: Vec<String> = Vec::new(); // The final board output (each row as a `String`)

    // Iterate over each row (`line` is row index, `byte` is the row's content)
    for (line, byte) in byte_board.iter().enumerate() {
        let mut row_result = String::new(); // Store the updated row after processing

        // Iterate over each column in the row
        for (column, _c) in byte.iter().enumerate() {
            if byte_board[line][column] == b'*' {  // If current cell is a mine
                row_result.push('*');             // Keep it as a '*'
                continue;                         // Skip checking neighbors
            }

            let mut count = 0; // Initialize the mine count around this cell

            // Loop through all 8 possible neighboring directions
            for (dx, dy) in NEIGHBORS {
                let new_row = (line as isize) + dy;  // Calculate neighbor row
                let new_column = (column as isize) + dx; // Calculate neighbor column

                // Ensure new_row and new_column are within bounds of the board
                if new_row >= 0
                    && new_column >= 0
                    && new_row < number_of_rows as isize
                    && new_column < number_of_columns as isize
                {
                    let cell_value = byte_board[new_row as usize][new_column as usize]; // Get neighbor value
                    dbg!(new_row, new_column, cell_value);  // Debugging: Check the values

                    if cell_value == b'*' { // If the neighbor is a mine
                        count += 1;         // Increase the mine count
                    }
                }
            }

            if count > 0 {
                row_result.push(std::char::from_digit(count as u32, 10).unwrap()); // Convert count to char and append
            } else {
                row_result.push(' '); // No mines nearby, keep it as a space
            }
        }

        final_board.push(row_result); // Add the processed row to the final board
    }

    final_board // Return the final board
}
