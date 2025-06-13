
fn traverse_board(size: usize) -> bool {
    size == 1 || size >= 5
}
pub fn task2_solve(number_of_boards: usize, sizes: Vec<usize>) -> () {
    let res : Vec<String> = sizes
        .into_iter()
        .take(number_of_boards)
        .map(|n| if traverse_board(n) { "yes".to_string() } else { "no".to_string() })
        .collect();
    println!("{:?}", res);
}

fn is_safe(x: i32, y: i32, n: i32, board: &Vec<Vec<i32>>) -> bool {
    x >= 0 && y >= 0 && x < n && y < n && board[x as usize][y as usize] == -1
}

fn knight_tour_util(
    x: i32,
    y: i32,
    step: i32,
    n: i32,
    board: &mut Vec<Vec<i32>>,
    dx: &[i32; 8],
    dy: &[i32; 8],
) -> bool {
    // If we've visited all squares
    if step == n * n {
        return true;
    }

    // Try all 8 knight moves
    for i in 0..8 {
        let nx = x + dx[i];
        let ny = y + dy[i];
        if is_safe(nx, ny, n, board) {
            board[nx as usize][ny as usize] = step;
            if knight_tour_util(nx, ny, step + 1, n, board, dx, dy) {
                return true;
            }
            // backtrack
            board[nx as usize][ny as usize] = -1;
        }
    }
    false
}

/// Attempts to find one Knight's Tour on an n×n board,
/// starting from (0,0). Returns Some(board) if successful,
/// where board[i][j] is the step index at which the knight visits
/// square (i,j); otherwise returns None.
pub fn knight_tour(n: i32) -> Option<Vec<Vec<i32>>> {
    // Initialize board with -1
    let mut board = vec![vec![-1; n as usize]; n as usize];

    // All 8 possible knight moves
    let dx = [2, 1, -1, -2, -2, -1, 1, 2];
    let dy = [1, 2, 2, 1, -1, -2, -2, -1];

    // Start from (0,0)
    board[0][0] = 0;
    if knight_tour_util(0, 0, 1, n, &mut board, &dx, &dy) {
        Some(board)
    } else {
        None
    }
}
