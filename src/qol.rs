pub fn display_bit_board(x: &u64) {
    let mut text = String::new();

    for j in (0..8).rev() {
        for i in 0..8 {
            text.push_str(&format!("{}", (x >> (8 * j + i)) & 1));
        }
        text.push('\n');
    }

    println!("{text}\n\n");
}
