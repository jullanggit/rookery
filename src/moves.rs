use crate::{
    board::{PType, State},
    functions::{pdep, remove_border},
    pext,
    qol::display_bit_board,
    BLOCKED_BISHOP, BLOCKED_ROOK, EMPTY_PSEUDO_BISHOP, EMPTY_PSEUDO_KING, EMPTY_PSEUDO_KNIGHT,
    EMPTY_PSEUDO_ROOK,
};

pub fn get_legal_move_board(pos_idx: u64, piece: PType, white: bool, state: &State) -> u64 {
    let board = if white { state.white } else { state.black };

    match piece {
        PType::Pawn => pawn_moves(pos_idx, white, state),
        PType::Rook => {
            return BLOCKED_ROOK[pos_idx as usize][pext(
                state.all_pieces,
                remove_border(EMPTY_PSEUDO_ROOK[pos_idx as usize]),
            ) as usize];
        }
        PType::Knight => EMPTY_PSEUDO_KNIGHT[pos_idx as usize] & !board.all,
        PType::Bishop => {
            return BLOCKED_BISHOP[pos_idx as usize][pext(
                state.all_pieces,
                remove_border(EMPTY_PSEUDO_BISHOP[pos_idx as usize]),
            ) as usize];
        }
        PType::King => EMPTY_PSEUDO_KING[pos_idx as usize] & !board.all,
        PType::Queen => {
            return BLOCKED_BISHOP[pos_idx as usize][pext(
                state.all_pieces,
                remove_border(EMPTY_PSEUDO_BISHOP[pos_idx as usize]),
            ) as usize]
                | BLOCKED_ROOK[pos_idx as usize][pext(
                    state.all_pieces,
                    remove_border(EMPTY_PSEUDO_ROOK[pos_idx as usize]),
                ) as usize];
        }
    }
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
        ((pos_board & !a) << 7 | ((pos_board & !h) << 9)) & state.black.all
    } else {
        (pos_board & !h) >> 7 | ((pos_board & !a) >> 9) & state.white.all
    };

    moves
}
