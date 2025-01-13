mod pieces;
mod board;
mod moves;
mod math;

#[cfg(test)]
mod tests {
    use super::*;

    use math::V2;

    use pieces::{ChessPiece, ChessPieceColor};
    use moves::{ChessMove, ChessMoveOption};
    use board::{ChessTile, ChessBoard};

    #[test]
    fn vec_2_test() {
        let vec: V2 = V2 { x: 6, y: 9 };
        assert_eq!(6, vec.x);
        assert_eq!(9, vec.y);
    }

    #[test]
    fn chess_move_test() {
        let src: V2 = V2 {x: 0, y: 3};
        let dst: V2 = V2 {x: 1, y: 3};
        
        let chess_move = ChessMove::new(&src, &dst, ChessMoveOption::Quiet);

        assert_eq!(src, chess_move.src());
        assert_eq!(dst, chess_move.dst());
        assert_eq!(ChessMoveOption::Quiet, chess_move.option());
    }
    
    #[test]
    fn chess_white_tile_test() {
        let piece: Option<ChessPiece> = Some(ChessPiece::Pawn);
        let color: Option<ChessPieceColor> = Some(ChessPieceColor::White);

        let tile = ChessTile::new(piece, color).expect("could not create tile");

        assert_eq!(tile.piece(), Some(ChessPiece::Pawn));
        assert_eq!(tile.color(), Some(ChessPieceColor::White));
    }

    #[test]
    fn chess_black_tile_test() {
        let piece: Option<ChessPiece> = Some(ChessPiece::Pawn);
        let color: Option<ChessPieceColor> = Some(ChessPieceColor::Black);

        let tile = ChessTile::new(piece, color).expect("could not create tile");

        assert_eq!(tile.piece(), Some(ChessPiece::Pawn));
        assert_eq!(tile.color(), Some(ChessPieceColor::Black));
    }

    #[test]
    fn chess_empty_tile_test() {
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
                assert_eq!(chess_board.get_tile(&src).color(), Some(ChessPieceColor::White));
            }
        }

        // bottom row is same
        for y in 6..8 {
            for x in 0..8 {
                let src = V2 { x, y };
                assert_eq!(chess_board.get_tile(&src).color(), Some(ChessPieceColor::Black));
            }
        }
    }

    #[test]
    fn chess_board_put_test() {
        let mut chess_board = ChessBoard::new(ChessPieceColor::White);

        let src = V2 { x: 0, y: 1 };
        let dst = V2 { x: 0, y: 2 };

        chess_board.put_tile(&src, &dst);

        assert_eq!(chess_board.get_tile(&src).color(), None);
        assert_eq!(chess_board.get_tile(&dst).color(), Some(ChessPieceColor::Black));
    }

}
