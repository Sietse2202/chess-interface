//! # chess-interface
//! I made this just for a small chess engine competition so that we have one unified trait to
//! implement when making our engines.

#[cfg(all(feature = "implementer", feature = "user"))]
compile_error!("You can't have both `implementer` and `user` features enabled.");

#[cfg(not(any(feature = "implementer", feature = "user")))]
compile_error!("You can't have both `implementer` and `user` features disabled.");

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use chess::{Board, ChessMove};
use std::time::Duration;

/// Trait that chess engines can implement.
pub trait ChessEngine: Default {
    /// Name of the engine
    const NAME: &'static str;

    /// Authors of the engine, colon-separated.
    const AUTHORS: &'static str;

    /// Version of the engine
    const VERSION: &'static str;

    /// Method for when the other player is making a move,
    fn ponder(&mut self, board: &Board, cancel: PonderCancel) -> PonderResult {
        _ = (board, cancel);
        PonderResult::Cancelled
    }

    /// This function will be used to let the engines make moves in the game.
    fn next_move(
        &mut self,
        board: &Board,
        remaining_time: Duration,
        increment: Duration,
    ) -> ChessMove;
}

/// Struct that tells an engine if the `ponder` is canceled.
#[derive(Debug, Clone)]
pub struct PonderCancel {
    cancelled: Arc<AtomicBool>,
}

impl PonderCancel {
    /// Create a new `PonderCancel`
    #[cfg(feature = "user")]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Cancel the `ponder`
    #[cfg(feature = "user")]
    #[inline]
    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// Check if the `ponder` is cancelled
    #[cfg(feature = "implementer")]
    #[inline]
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Enum for the `ponder` to return.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum PonderResult {
    /// Ponder is cancelled. Returned if the engine user calls for a cancel, or if the engine wants
    /// to abort the pondering
    Cancelled,
    /// Move to premove. Returned when you found a move you want to play during pondering.
    Move(ChessMove),
}
