#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect { x: x, y: y, w: w, h: h }
    }

    pub fn empty() -> Rect {
        Rect { x: 0_f32, y: 0_f32, w: 0_f32, h: 0_f32 }
    }

    pub fn intersects(&self, r: &Rect) -> bool {
        self.x + self.w >= r.x && self.x < r.x + r.w && self.y + self.h >= r.y && self.y < r.y + r.h
    }

    pub fn collision(&self, r: &Rect) -> Rect {
        let left = self.x.max(r.x);
        let right = (self.x + self.w).min(r.x + r.w);
        let top = self.y.max(r.y);
        let bottom = (self.y + self.h).min(r.y + r.h);

        if left <= right && top <= bottom {
            Rect::new(left, top, right - left, bottom - top)
        } else {
            Rect::empty()
        }
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    pub fn empty() -> Point {
        Point { x: 0_f32, y: 0_f32 }
    }
}
