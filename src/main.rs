mod task1;
mod task2;

use task1::{taks1_solve};
use task2::{task2_solve};
fn main() {
    // Task 1 section
    let cypher = String::from("UDMHUHCHUHBH");
    taks1_solve(cypher);

    let number_of_boards = 10;
    let boards = vec![1,2,3,4];
    task2_solve(number_of_boards, boards);
}
