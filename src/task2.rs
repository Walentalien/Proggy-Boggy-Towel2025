
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