// first we need to try out every single combination
// and for shift of 25 we can see that it works
pub fn task1_helper(cipher: &str) {
    for shift in 0..26 {
        let decrypted: String = cipher
            .bytes()
            .map(|b| {
                    ((b - b'A' + 26 - shift) % 26 + b'A') as char
            })
            .collect();

        println!("Shift {}: {}", shift, decrypted);
    }
}

pub fn taks1_solve(cypher: String) -> () {
    let shift  = 25;
    let deciphered: String = cypher
        .bytes()
        .map(
    |char| {
        ((char-b'A'+26-shift)%26 + b'A') as char
    }
        ).collect();
    println!("Shift {}: {}", shift, deciphered);
}