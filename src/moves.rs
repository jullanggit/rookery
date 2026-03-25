use crate::{
    board::{PType, State},
    functions::extract_bits,
    qol::display_bit_board,
};

pub const PSEUDO_ROOK: [u64; 64] = generate_pseudo_rook();
pub const PSEUDO_BISHOP: [u64; 64] = generate_pseudo_bishop();
pub const PSEUDO_KNIGHT: [u64; 64] = generate_pseudo_knight();
pub const PSEUDO_KING: [u64; 64] = generate_pseudo_king();

pub fn get_legal_move(pos_idx: u64, piece: PType, white: bool, state: &State) -> Vec<u16> {
    let mut moves = Vec::new();

    let board = if white { state.white } else { state.black };

    if piece.sliding() {
        // let moves =
        // let blockers_idx = extract_bits(, state.all_pieces)
    }

    moves
}

pub fn pawn_moves(pos_idx: u64, white: bool, state: &State) -> u64 {
    let mut moves: u64 = 0;

    let start_row_after_1st_move: u64 = if white { 0xFF << 16 } else { 0xFF << (8 * 5) };

    let pos_board = 1 << pos_idx;

    let a: u64 = 0x0101010101010101;
    let h: u64 = a << 7;

    moves |= if white {
        pos_board << 8
    } else {
        pos_board >> 8
    } & !state.all_pieces;

    moves |= if white {
        (moves & start_row_after_1st_move & !state.all_pieces) << 8
    } else {
        (moves & start_row_after_1st_move & !state.all_pieces) >> 8
    };

    moves |= if white {
        ((pos_board & !a) << 7 | ((pos_board & !h) << 9)) & !state.white.all
    } else {
        (pos_board & !h) >> 7 | ((pos_board & !a) >> 9) & !state.black.all
    };

    moves
}

const fn generate_pseudo_king() -> [u64; 64] {
    let mut attacks: [u64; 64] = [0; 64];

    let mut sq = 0;
    while sq < 64 {
        let mut board: u64 = 0;

        let a: u64 = 0x0101010101010101;
        let h: u64 = a << 7;

        let p = 1 << sq as u64;

        board |= (p & !h) << 1;
        board |= (p & !a) >> 1;
        board |= p << 8;
        board |= p >> 8;

        board |= (p & !h) >> 7;
        board |= (p & !a) << 7;
        board |= (p & !a) >> 9;
        board |= (p & !h) << 9;

        attacks[sq] = board;

        sq += 1;
    }

    attacks
}

const fn generate_pseudo_knight() -> [u64; 64] {
    let mut attacks: [u64; 64] = [0; 64];

    let mut sq = 0;
    while sq < 64 {
        let mut board: u64 = 0;

        let a: u64 = 0x0101010101010101;
        let ab: u64 = a | a << 1;
        let h: u64 = a << 7;
        let gh: u64 = h | h >> 1;

        let p = 1 << sq as u64;

        board |= (p & !ab) << 6;
        board |= (p & !gh) << 10;
        board |= (p & !a) << 15;
        board |= (p & !h) << 17;

        board |= (p & !gh) >> 6;
        board |= (p & !ab) >> 10;
        board |= (p & !h) >> 15;
        board |= (p & !a) >> 17;

        attacks[sq] = board;

        sq += 1;
    }

    attacks
}

const fn generate_pseudo_bishop() -> [u64; 64] {
    let mut ray_attacks = [0; 64];

    let mut sq = 0;
    while sq < 64 {
        let maindia: u64 = 0x8040201008040201;
        let diag = 8 * (sq as i32 & 7) - (sq as i32 & 56);
        let nort = -diag & (diag >> 31);
        let sout = diag & (-diag >> 31);
        ray_attacks[sq] = (maindia >> sout) << nort & !(1 << sq);
        sq += 1;
    }

    sq = 0;
    while sq < 64 {
        let maindia: u64 = 0x0102040810204080;
        let diag = 56 - 8 * (sq as i32 & 7) - (sq as i32 & 56);
        let nort = -diag & (diag >> 31);
        let sout = diag & (-diag >> 31);

        ray_attacks[sq] |= (maindia >> sout) << nort & !(1 << sq);
        sq += 1
    }

    ray_attacks
}

const fn generate_pseudo_rook() -> [u64; 64] {
    let mut ray_attacks = [0; 64];

    let mut sq = 0;
    while sq < 64 {
        ray_attacks[sq] = 0xFF << (sq & 56) & !(1 << sq);
        sq += 1;
    }

    sq = 0;
    while sq < 64 {
        ray_attacks[sq] |= 0x0101010101010101 << (sq & 7) & !(1 << sq);
        sq += 1;
    }

    ray_attacks
}
