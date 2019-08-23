#[derive(Default)]
pub struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

impl From<(i32, i32)> for Pos {
    fn from(source: (i32, i32)) -> Self {
        let (x, y) = source;
        Pos::new(x, y)
    }
}

#[derive(Default)]
pub struct Size {
    w: i32,
    h: i32
}

impl Size {
    pub fn new(w: i32, h: i32) -> Size {
        Size { w, h }
    }
}

impl From<(i32, i32)> for Size {
    fn from(source: (i32, i32)) -> Self {
        let (w, h) = source;
        Size::new(w, h)
    }
}

pub struct Area {
    pos: Pos,
    size: Size
}

impl Area {
    pub fn new<P, S>(pos: P, size: S) -> Area
        where P: Into<Pos>, S: Into<Size> {
        Area {
            pos: pos.into(),
            size: size.into()
        }
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

impl From<(i32, i32, i32, i32)> for Area {
    fn from(source: (i32, i32, i32, i32)) -> Self {
        let (x, y, w, h) = source;
        Area::new(
            Pos::new(x, y),
            Size::new(w, h)
        )
    }
}

impl From<Pos> for Area {
    fn from(pos: Pos) -> Self {
        Area::new(
            pos,
            Size::default()
        )
    }
}

impl From<Size> for Area {
    fn from(size: Size) -> Self {
        Area::new(
            Pos::default(),
            size
        )
    }
}

impl From<(i32, i32)> for Area {
    fn from(size: (i32, i32)) -> Self {
        Area::new(
            Pos::default(),
            Size::from(size)
        )
    }
}
