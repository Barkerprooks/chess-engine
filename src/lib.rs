pub mod pieces;
pub mod board;
pub mod moves;
pub mod math;

pub use pieces::{ChessPiece, ChessPieceColor};
pub use moves::{ChessMove, ChessMoveExt};
pub use board::{ChessTile, ChessBoard};
pub use math::V2;

#[cfg(test)]
mod tests {
    use super::*;

    use pieces::{ChessPiece, ChessPieceColor};
    use moves::{ChessMove, ChessMoveExt};
    use board::{ChessTile, ChessBoard};
    use math::V2;

    const TEST_LAYOUT: [u8; 64] = [
        2, 0, 0, 0, 0, 0, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        1, 1, 0, 0, 1, 1, 1, 1,
        2, 3, 4, 5, 6, 4, 3, 2
    ];

    #[test]
    fn chess_move_test() {
        let src = V2 {x: 0, y: 1}; // move enemy pawn
        let dst = V2 {x: 0, y: 2}; // one forward
        
        let chess_move = ChessMove::raw(&src, &dst, ChessMoveExt::Quiet as u16);

        assert_eq!(src, chess_move.src());
        assert_eq!(dst, chess_move.dst());
        assert_eq!(ChessMoveExt::Quiet, chess_move.ext());
    }
    
    #[test]
    fn chess_tile_white_test() {
        let piece = Some(ChessPiece::Pawn);
        let color = Some(ChessPieceColor::White);

        let tile = ChessTile::new(piece, color).expect("could not create tile");

        assert_eq!(tile.piece(), Some(ChessPiece::Pawn));
        assert_eq!(tile.color(), Some(ChessPieceColor::White));
    }

    #[test]
    fn chess_tile_black_test() {
        let piece = Some(ChessPiece::Pawn);
        let color = Some(ChessPieceColor::Black);

        let tile = ChessTile::new(piece, color).expect("could not create tile");

        assert_eq!(tile.piece(), Some(ChessPiece::Pawn));
        assert_eq!(tile.color(), Some(ChessPieceColor::Black));
    }

    #[test]
    fn chess_tile_empty_test() {
        let tile =  ChessTile::new(None, None).expect("could not create tile");

        assert_eq!(tile.piece(), None);
        assert_eq!(tile.color(), None);
    }

    #[test]
    fn chess_board_new_test() {
        let chess_board = ChessBoard::new(ChessPieceColor::Black);

        // make sure top row is opposite
        for y in 0..2 {
            for x in 0..8 {
                let src = V2 { x, y };
                assert_eq!(chess_board.tile(&src).color(), Some(ChessPieceColor::White));
            }
        }

        // bottom row is same
        for y in 6..8 {
            for x in 0..8 {
                let src = V2 { x, y };
                assert_eq!(chess_board.tile(&src).color(), Some(ChessPieceColor::Black));
            }
        }
    }

    #[test]
    fn chess_board_take_turn_pawn_move() {
        let mut chess_board = ChessBoard::new(ChessPieceColor::White);

        let src = V2 { x: 0, y: 1 };
        let dst = V2 { x: 0, y: 2 };

        assert!(chess_board.take_turn(&src, &dst));

        assert_eq!(chess_board.tile(&src).color(), None);
        assert_eq!(chess_board.tile(&dst).color(), Some(ChessPieceColor::Black));

        assert_eq!(
            chess_board.last_turn().expect("no last move").ext(), 
            ChessMoveExt::Quiet
        );
    }

    // TODO: test pawn illegal moves 

    #[test]
    fn chess_board_take_turn_pawn_double_move() {
        let mut chess_board = ChessBoard::new(ChessPieceColor::Black);

        let src = V2 { x: 0, y: 1 }; // move enemy pawn twice
        let dst = V2 { x: 0, y: 3 };

        assert!(chess_board.take_turn(&src, &dst));

        assert_eq!(chess_board.tile(&src).color(), None);
        assert_eq!(chess_board.tile(&dst).color(), Some(ChessPieceColor::White));
        
        assert_eq!(
            chess_board.last_turn().expect("no last move").ext(), 
            ChessMoveExt::DoublePawnPush
        );
    }

    // TODO: test pawn double move illegal

    #[test]
    fn chess_board_take_turn_pawn_attack() {
        let mut chess_board = ChessBoard::new(ChessPieceColor::Black);
        let mut src = V2 { x: 0, y: 1 }; // starting enemy pawn

        let dsts = [
            V2 { x: 0, y: 3 }, // move forward 2 (y = 4)
            V2 { x: 0, y: 4 }, // move forward 1 (y = 5)
            V2 { x: 0, y: 5 }, // move forward 1 (y = 6, able to attack)
            V2 { x: 1, y: 6 } // attack
        ];

        for dst in dsts {
            chess_board.take_turn(&src, &dst);
            src = dst;
        }

        assert_eq!(
            chess_board.last_turn().expect("no last move").ext(),
            ChessMoveExt::Captures
        )
    }

    #[test]
    fn chess_board_take_turn_rook_x_axis() {
        let mut chess_board = ChessBoard::from_layout(ChessPieceColor::White, TEST_LAYOUT);

        let src = V2 { x: 0, y: 0 };
        let dst = V2 { x: 4, y: 0 };

        assert!(chess_board.take_turn(&src, &dst));
    }

    #[test]
    fn chess_board_take_turn_rook_y_axis() {
        let mut chess_board = ChessBoard::from_layout(ChessPieceColor::White, TEST_LAYOUT);

        let move_pawn_src = V2 { x: 0, y: 1 };
        let move_pawn_dst = V2 { x: 0, y: 3 };

        chess_board.take_turn(&move_pawn_src, &move_pawn_dst);

        let src = V2 { x: 0, y: 0 };
        let dst = V2 { x: 0, y: 2 };

        assert!(chess_board.take_turn(&src, &dst));
    }

    // TODO: test invalid rook movements

    #[test]
    fn chess_board_take_turn_knight_move() {
        let mut chess_board = ChessBoard::new(ChessPieceColor::Black);

        let src = V2 { x: 1, y: 0 };
        let dst = V2 { x: 2, y: 2 };

        assert!(chess_board.take_turn(&src, &dst));
    }

    #[test]
    fn chess_board_take_turn_knight_invalid_move() {
        let mut chess_board = ChessBoard::new(ChessPieceColor::White);

        let src = V2 { x: 1, y: 0 };
        let dst = V2 { x: 2, y: 1 };

        assert_eq!(chess_board.take_turn(&src, &dst), false);
    }

    #[test]
    fn chess_board_take_turn_bishop() {
        let mut chess_board = ChessBoard::from_layout(ChessPieceColor::White, TEST_LAYOUT);
        
        let src = V2 { x: 2, y: 7 };
        let dst = V2 { x: 4, y: 5 };

        assert!(chess_board.take_turn(&src, &dst));
    }

    #[test]
    fn chess_board_take_turn_queen_move_plus() {
        let mut chess_board = ChessBoard::from_layout(ChessPieceColor::Black, TEST_LAYOUT);

        let src = V2 { x: 3, y: 7 };
        let dst = V2 { x: 3, y: 4 };

        assert!(chess_board.take_turn(&src, &dst));
    }

    #[test]
    fn chess_board_take_turn_queen_move_diag() {
        let mut chess_board = ChessBoard::from_layout(ChessPieceColor::White, TEST_LAYOUT);

        let src = V2 { x: 3, y: 7 };
        let dst = V2 { x: 0, y: 4 };

        assert!(chess_board.take_turn(&src, &dst));
    }

    #[test]
    fn chess_board_take_turn_queen_move_invalid() {
        let mut chess_board = ChessBoard::from_layout(ChessPieceColor::White, TEST_LAYOUT);

        let src = V2 { x: 3, y: 7 };
        let dst = V2 { x: 2, y: 5 };

        assert_eq!(chess_board.take_turn(&src, &dst), false);
    }

    #[test]
    fn chess_board_take_turn_king_move() {
        let mut chess_board = ChessBoard::from_layout(ChessPieceColor::White, TEST_LAYOUT);

        let src = V2 { x: 4, y: 7 };
        let dst = V2 { x: 3, y: 6 };

        assert!(chess_board.take_turn(&src, &dst));
    }

    #[test]
    fn chess_board_take_turn_king_move_invalid() {
        let mut chess_board = ChessBoard::from_layout(ChessPieceColor::White, TEST_LAYOUT);

        let src = V2 { x: 4, y: 7 };
        let dst = V2 { x: 4, y: 6 };

        assert_eq!(chess_board.take_turn(&src, &dst), false);
    }
}
