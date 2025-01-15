use crate::pieces::{ChessPiece, ChessPieceColor};
use crate::moves::{ChessMove};
use crate::math::V2;

const DEFAULT_CHESS_BOARD_MAP: [u8; 64] = [
    2, 3, 4, 5, 6, 4, 3, 2,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    2, 3, 4, 5, 6, 4, 3, 2
];

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChessTile (u8);

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct ChessBoard {
    turn: u8, // keeping track of the current turn
    tiles: [ChessTile; 64], // 8x8 board
    moves: [ChessMove; 50], // game ends at 50 moves
    player_color: ChessPieceColor
}

#[allow(dead_code)]
impl ChessTile {
    pub fn new(piece: Option<ChessPiece>, color: Option<ChessPieceColor>) -> Result<Self, &'static str> {
        match piece {
            Some(piece) => match color {
                Some(ChessPieceColor::Black) => Ok(ChessTile(piece as u8 | 1 << 3)),
                Some(ChessPieceColor::White) => Ok(ChessTile(piece as u8 | 2 << 3)),
                None => Err("occupied tile requires a color")
            },
            None => Ok(ChessTile(0))
        }
    }

    pub fn has_moved(&self) -> bool {
        (self.0 >> 5) & 1 == 1
    }

    pub fn moved(&self) -> Self {
        ChessTile(self.0 | 1 << 5) // set the 6th bit to indicate the piece has moved
    }

    pub fn piece(&self) -> Option<ChessPiece> {
        ChessPiece::from(self.0 & 0x7)
            .expect("not a valid chess piece")
    }

    pub fn color(&self) -> Option<ChessPieceColor> {
        match (self.0 >> 3) & 3 {
            1 => Some(ChessPieceColor::Black),
            2 => Some(ChessPieceColor::White),
            _ => None
        }
    }

    pub fn from_layout(index: usize, value: &u8, colors: (ChessPieceColor, ChessPieceColor)) -> ChessTile {
        let color = match index {
            0..16 => Some(colors.1),
            48..64 => Some(colors.0),
            _ => None
        };
    
        let piece = ChessPiece::from(*value)
            .expect("not a valid piece");
    
        ChessTile::new(piece, color)
            .expect("error creating tile")
    }
}

#[allow(dead_code)]
impl ChessBoard {
    pub fn from_layout(player_color: ChessPieceColor, chess_board_map: [u8; 64]) -> Self {
        let opponent_color = match player_color {
            ChessPieceColor::White => ChessPieceColor::Black,
            ChessPieceColor::Black => ChessPieceColor::White
        };

        let colors = (player_color, opponent_color);

        ChessBoard {
            player_color: player_color,
            tiles: chess_board_map.iter()
                .enumerate()
                .map(|(index, value)| ChessTile::from_layout(index, value, colors))
                .collect::<Vec<ChessTile>>()
                .try_into()
                .unwrap(),
            moves: [ChessMove(0); 50],
            turn: 0
        }
    } 

    pub fn new(player_color: ChessPieceColor) -> Self {
        Self::from_layout(player_color, DEFAULT_CHESS_BOARD_MAP)
    }

    pub fn player_color(&self) -> ChessPieceColor {
        self.player_color
    }

    pub fn clear(&mut self, src: &V2) {
        let x = (*src).x as usize; 
        let y = (*src).y as usize;
        self.tiles[x+ 8 * y] = ChessTile(0)
    }

    pub fn place(&mut self, src: &V2, tile: ChessTile) {
        let x = (*src).x as usize; 
        let y = (*src).y as usize;
        self.tiles[x + 8 * y] = tile;
    }

    pub fn last_turn(&self) -> Option<&ChessMove> {
        match self.turn {
            0 => None,
            _ => Some(&self.moves[(self.turn - 1) as usize])
        }
    }

    pub fn tile(&self, src: &V2) -> ChessTile {
        let x = (*src).x as usize; 
        let y = (*src).y as usize;
        self.tiles[x+ 8 * y]
    }

    pub fn tile_pair(&self, src: &V2, dst: &V2) -> (ChessTile, ChessTile) {
        (self.tile(src), self.tile(dst))
    }

    pub fn take_turn(&mut self, src: &V2, dst: &V2) -> bool {
        if self.turn == 50 {
            return false;
        }

        // i would name this vaiable "move" but that's a keyword in rust
        match ChessMove::new(src, dst, *self) {
            Ok(movement) => {
                // make sure to set the moved bit
                self.place(dst, self.tile(src).moved());
                self.clear(src);
    
                self.moves[self.turn as usize] = movement;
                self.turn += 1;
    
                true
            },
            Err(_) => false
        }
    }

    pub fn show(&self) {
        println!();
        for y in 0..8 {
            for x in 0..8 {
                match self.tiles[x + 8 * y].piece() {
                    Some(piece) => {
                        let symbol = match piece {
                            ChessPiece::Pawn => 'p',
                            ChessPiece::Rook => 'r',
                            ChessPiece::Knight => 'k',
                            ChessPiece::Bishop => 'b',
                            ChessPiece::Queen => 'q',
                            ChessPiece::King => 'K'
                        };

                        let color = match self.tiles[x + 8 * y].color() {
                            Some(ChessPieceColor::Black) => 'B',
                            Some(ChessPieceColor::White) => 'W',
                            _ => '_'
                        };

                        print!("{}{}", color, symbol);
                    },
                    None => print!("__")
                };
            }
            println!();
        }
        println!();
    }
}