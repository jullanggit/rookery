use crate::{
    functions::{fen_pos_notation_to_sq_index, fen_positions_to_bitboards},
    moves::get_legal_move_board,
    qol::display_bit_board,
};

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub white: bool,
    pub _type: PType,
    pub position_idx: u64,
}

#[derive(Clone, Debug)]
pub struct State {
    pub white: ColorBoards,
    pub black: ColorBoards,
    pub all_pieces: u64,
    pub en_passant: u64,
    pub white_to_move: bool,
    pub half_move_clock: usize,
    pub full_move_clock: usize,
    pub pieces_list: Vec<Piece>,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct ColorBoards {
    pub pawn: u64,
    pub rook: u64,
    pub knight: u64,
    pub bishop: u64,
    pub queen: u64,
    pub king: u64,
    pub all: u64,
}

#[derive(Clone, Copy, Debug)]
pub enum PType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

// XX-0000-0000-0000
//    prom to   from
pub type Move = u16;

pub fn get_moves_from_move_board(mut move_board: u64, idx: u64) -> Vec<Move> {
    let mut moves = Vec::new();

    let mut indices = Vec::new();

    while move_board != 0 {
        // trailing_zeros() returns the index of the lowest set bit
        let index = move_board.trailing_zeros();
        indices.push(index);

        // Clear the lowest set bit (Brian Kernighan's algorithm)
        move_board &= move_board - 1;
    }

    for index in indices {
        let from = idx as u16;
        let to = (index as u16) << 4;
        moves.push(from | to);
    }

    moves
}

impl PType {
    pub fn sliding(&self) -> bool {
        match self {
            Self::Rook => true,
            Self::Bishop => true,
            Self::Queen => true,
            _ => false,
        }
    }
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
    pub fn from_fen(fen: &str) -> Self {
        let mut pieces = Vec::new();

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

        let (white, black) = fen_positions_to_bitboards(pos, &mut pieces);
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
            pieces_list: pieces,
        }
    }

    pub fn get_all_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for piece in &self.pieces_list {
            let board = get_legal_move_board(piece.position_idx, piece._type, piece.white, &self);
            moves.extend(get_moves_from_move_board(board, piece.position_idx));
        }

        moves
    }
}
