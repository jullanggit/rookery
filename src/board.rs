pub struct Board {
    white_boards: BitBoards,
    black_boards: BitBoards,
}

pub struct BitBoards {
    pawns: u64,
    rooks: u64,
    knights: u64,
    bishops: u64,
    king: u64,
    queen: u64,
    attack: u64,
}
