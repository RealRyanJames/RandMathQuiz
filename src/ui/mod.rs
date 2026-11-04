type Pos = u32;

pub struct UI {
    pub x: Pos,
    pub y: Pos,
    pub z: Pos,
}

pub trait Positions {
    fn get_x(&self) -> Pos;
    fn get_y(&self) -> Pos;
    fn get_z(&self) -> Pos;
}

impl Positions for UI {
    fn get_x(&self) -> Pos {
        self.x
    }

    fn get_y(&self) -> Pos {
        self.y
    }

    fn get_z(&self) -> Pos {
        self.z
    }
}
