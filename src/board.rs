use crate::functions::{fen_pos_notation_to_sq_index, fen_positions_to_bitboards};

pub struct State {
    pub white: ColorBoards,
    pub black: ColorBoards,
    pub all_pieces: u64,
    pub en_passant: u64,
    pub white_to_move: bool,
    pub half_move_clock: usize,
    pub full_move_clock: usize,
}

#[derive(Default)]
pub struct ColorBoards {
    pub pawn: u64,
    pub rook: u64,
    pub knight: u64,
    pub bishop: u64,
    pub queen: u64,
    pub king: u64,
    pub all: u64,
}

impl ColorBoards {
    pub fn update_all(&mut self) {
        self.all = self.pawn | self.rook | self.knight | self.bishop | self.queen | self.king;
    }
}

impl Default for State {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl State {
    fn from_fen(fen: &str) -> Self {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        let (pos, to_move, cast, en_pass, half_move, full_move) =
            (parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]);

        let en_passant = if en_pass != "-" {
            1 << fen_pos_notation_to_sq_index(en_pass)
        } else {
            0
        };

        let half_move_clock: usize = half_move
            .parse()
            .expect("Expected a half move clock number");
        let full_move_clock: usize = full_move
            .parse()
            .expect("Expected a full move clock number");

        let (white, black) = fen_positions_to_bitboards(pos);
        let all_pieces = white.all | black.all;

        let white_to_move = to_move == "w";

        Self {
            white,
            black,
            en_passant,
            white_to_move,
            all_pieces,
            half_move_clock,
            full_move_clock,
        }
    }
}
