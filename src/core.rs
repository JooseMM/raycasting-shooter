pub struct Vector {
    pub x: i8,
    pub y: i8,
}

pub struct Player {
    pub(crate) pos: Vector,
    pub direction: Vector,
}
