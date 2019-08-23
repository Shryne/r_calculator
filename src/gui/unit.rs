
pub struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

pub struct Size {
    w: i32,
    h: i32
}

impl Size {
    pub fn new(w: i32, h: i32) -> Size {
        Size { w, h }
    }
}

pub struct Area {
    pos: Pos,
    size: Size
}

impl Area {
    pub fn new(pos: Pos, size: Size) -> Area {
        Area {
            pos,
            size
        }
    }

    pub fn xywh(x: i32, y: i32, w: i32, h: i32) -> Area {
        Area::new(
            Pos::new(x, y),
            Size::new(w, h)
        )
    }

    pub fn x(&self) -> i32 {
        self.pos.x
    }

    pub fn y(&self) -> i32 {
        self.pos.y
    }

    pub fn w(&self) -> i32 {
        self.size.w
    }

    pub fn h(&self) -> i32 {
        self.size.h
    }
}
