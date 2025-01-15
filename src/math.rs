use crate::board::{ChessBoard, ChessTile};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct V2 {
    pub x: u16, 
    pub y: u16 
}

impl V2 {
    pub fn in_bounds(x: i8, y: i8) -> bool {
        x >= 0 && x < 8 && y >= 0 && y < 8
    }

    pub fn from_u16(x: u16, y: u16) -> Option<Self> {
        match x < 8 && y < 8 {
            true => Some(Self { x, y }),
            false => None
        }
    }

    pub fn from_i8(x: i8, y: i8) -> Option<Self> {
        match Self::in_bounds(x, y) {
            true => Some(Self { 
                x: x.try_into().unwrap(), 
                y: y.try_into().unwrap()
            }),
            false => None, // out of bounds
        }
    }

    pub fn get_offset(origin: &V2, x: i8, y: i8) -> Option<Self> {
        // returns none if not possible
        let offset_x = i8::try_from(origin.x).unwrap() + x;
        let offset_y = i8::try_from(origin.y).unwrap() + y;
        
        Self::from_i8(offset_x, offset_y)
    }
}

// searches in a plus / diagonal formation until piece is encountered
// if the piece is a different color, that tile is included
// otherwise it's omitted

// TODO: abstract the if statement clauses into its own function. each one is general
// enough to do this in plus and diag.

// there is probably a better way to do this

pub fn search_grid_plus(src: &V2, board: ChessBoard) -> Vec<V2> {
    let tile: ChessTile = board.tile(src);
    let mut moves: Vec<V2> = vec![];

    let mut to_head_x = i8::try_from(src.x).unwrap() - 1;
    let mut to_tail_x = i8::try_from(src.x).unwrap() + 1;
    let mut to_head_y = i8::try_from(src.y).unwrap() - 1;
    let mut to_tail_y = i8::try_from(src.y).unwrap() + 1;

    while to_head_x >= 0 || to_tail_x < 8 || to_head_y >= 0 || to_tail_y < 8 {
        if to_head_x >= 0 {
            let v2_head = V2 { x: u16::try_from(to_head_x).unwrap(), y: src.y };
            if board.tile(&v2_head).color() == tile.color() {
                to_head_x = -1;
            } else {
                moves.push(v2_head);
                to_head_x -= 1;
            }
        }
        if to_tail_x < 8 {
            let v2_tail = V2 { x: u16::try_from(to_tail_x).unwrap(), y: src.y };
            if board.tile(&v2_tail).color() == tile.color() {
                to_tail_x = 8;
            } else {
                moves.push(v2_tail);
                to_tail_x += 1;
            }
        }
        if to_head_y >= 0 {
            let v2_head = V2 { x: src.x, y: u16::try_from(to_head_y).unwrap() };
            if board.tile(&v2_head).color() == tile.color() {
                to_head_y = -1;
            } else {
                moves.push(v2_head);
                to_head_y -= 1;
            }
        }
        if to_tail_y < 8 {
            let v2_tail = V2 { x: src.x, y: u16::try_from(to_tail_y).unwrap() };
            if board.tile(&v2_tail).color() == tile.color() {
                to_tail_y = 8;
            } else {
                moves.push(v2_tail);
                to_tail_y += 1;
            }
        }
    }

    moves
}


pub fn search_grid_diag(src: &V2, board: ChessBoard) -> Vec<V2> {
    let tile_color = board.tile(src).color();
    let mut moves: Vec<V2> = vec![];

    let mut to_head_u = V2::get_offset(src, -1, -1);
    let mut to_head_d = V2::get_offset(src, -1,  1);
    let mut to_tail_u = V2::get_offset(src,  1, -1);
    let mut to_tail_d = V2::get_offset(src,  1,  1);

    while to_head_u != None || to_head_d != None || to_tail_u != None || to_tail_d != None {
        if to_head_u != None {
            let v2 = to_head_u.unwrap();
            if board.tile(&v2).color() == tile_color {
                to_head_u = None;
            } else {
                moves.push(v2);
                to_head_u = V2::get_offset(&v2, -1, -1);
            }
        }
        if to_head_d != None {
            let v2 = to_head_d.unwrap();
            if board.tile(&v2).color() == tile_color {
                to_head_d = None;
            } else {
                moves.push(v2);
                to_head_d = V2::get_offset(&v2, -1,  1);
            }
        }
        if to_tail_u != None {
            let v2 = to_tail_u.unwrap();
            if board.tile(&v2).color() == tile_color {
                to_tail_u = None;
            } else {
                moves.push(v2);
                to_tail_u = V2::get_offset(&v2,  1, -1);
            }
        }
        if to_tail_d != None {
            let v2 = to_tail_d.unwrap();
            if board.tile(&v2).color() == tile_color {
                to_tail_d = None;
            } else {
                moves.push(v2);
                to_tail_d = V2::get_offset(&v2,  1,  1);
            }
        }
    }

    moves
}