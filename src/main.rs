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

use std::env;
use std::fs;
use std::path::Path;
use std::time::Duration;
use std::time::Instant;

include!(concat!(env!("OUT_DIR"), "/tables.rs"));

fn main() {
    let mut state = State::default();

    let all_moves = state.get_all_moves();
}
