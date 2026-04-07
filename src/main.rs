#![allow(long_running_const_eval)]
#![allow(unused)]

mod board;
mod constants;
mod functions;
mod moves;
mod qol;

use board::*;
use constants::*;
use functions::*;
use moves::*;
use qol::*;

fn main() {
    let mut state = State::default();
    dbg!(pext(state.all_pieces, remove_border(EMPTY_PSEUDO_ROOK[0])));

    display_bit_board(&remove_border(EMPTY_PSEUDO_ROOK[0]));

    display_bit_board(&get_legal_move_board(0, PType::Rook, true, &state));

    display_bit_board(
        &BLOCKED_ROOK[0][pext(state.all_pieces, remove_border(EMPTY_PSEUDO_ROOK[0])) as usize],
    );

    let all_moves = state.get_all_moves();

    // for _move in all_moves {
    //     display_move(_move);
    // }
}
