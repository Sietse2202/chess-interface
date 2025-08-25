//! # chess-interface
//! I made this just for a small chess engine competition so that we have one unified trait to
//! implement when making our engines.

use chess::{Board, ChessMove};

/// Trait that chess engines can implement.
pub trait ChessEngine: Default {
    /// Name of the engine
    const NAME: &'static str;
    
    /// Authors of the engine, colon-separated.
    const AUTHORS: &'static str;
    
    /// Version of the engine
    const VERSION: &'static str;

    /// This function will be used to let the engines make moves in the game.
    fn next_move(&mut self, board: &Board) -> ChessMove;
}
