# chess-interface
Rust Library for a small chess engine competition

# How to use?
```rust
use chess_interface::ChessEngine;

struct MyEngine;

impl ChessEngine for MyEngine {
    const NAME: &str = "MyEngine";
    const AUTHORS: [&str] = ["Sietse"];
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    fn next_move(&mut self, board: Board) -> ChessMove {
        todo!()
    }
}
```

# License
THe Unlicense, do with this code whatever you want, it's barely even code.
