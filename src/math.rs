#[derive(Copy, Clone, Debug, PartialEq)]
pub struct V2 {
    pub x: u16, 
    pub y: u16 
}

impl V2 {
    pub fn from_i8(x: i8, y: i8) -> Option<Self> {
        match x < 0 || x > 7 || y < 0 || y > 7 {
            true => None, // out of bounds
            false => Some(Self { 
                x: x.try_into().unwrap(), 
                y: y.try_into().unwrap()
            })
        }
    }

    pub fn get_offset(origin: &V2, x: i8, y: i8) -> Option<Self> {
        // returns none if not possible
        let offset_x = i8::try_from(origin.x).unwrap() + x;
        let offset_y = i8::try_from(origin.y).unwrap() + y;
        
        Self::from_i8(offset_x, offset_y)
    }
}