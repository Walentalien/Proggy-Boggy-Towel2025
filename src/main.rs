mod task1;
mod task2;

use task1::{task1_solve};
use task2::{task2_solve};
use crate::task2::knight_tour;

fn main() {
    // Task 1 section
    let cypher = String::from("UDMHUHCHUHBH");
    task1_solve(cypher);

    // ===== TASK 2 ======
    let number_of_boards = 10;
    let boards = vec![1,2,3,4];
    task2_solve(number_of_boards, boards);

    for n in 1..20 {
        match knight_tour(n) {
            Some(board) => {
                for row in board {
                    for &val in &row {
                        print!("{:3} ", val);
                    }
                    println!();
                }
            }
            None => {
                println!("No tour found for a {}×{} board.", n, n);
            }
        }
    }
}
