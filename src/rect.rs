#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    pub fn intersects(&self, r: &Rect) -> bool {
        // Strict collision
        self.x + self.w > r.x && self.x < r.x + r.w && self.y + self.h > r.y && self.y < r.y + r.h
    }

    pub fn collision(&self, r: &Rect) -> Rect {
        let left = self.x.max(r.x);
        let right = (self.x + self.w).min(r.x + r.w);
        let top = self.y.max(r.y);
        let bottom = (self.y + self.h).min(r.y + r.h);

        if left <= right && top <= bottom {
            Rect::new(left, top, right - left, bottom - top)
        } else {
            Rect::default()
        }
    }

    pub fn distance(&self, &r: &Rect) -> f32 {
        if r.x + r.w > self.x && r.x < self.x + self.w {
            return (r.y - self.y - self.h).max(self.y - r.y - r.h);
        }
        if r.y + r.h > self.y && r.y < self.y + self.h {
            return (r.x - self.x - self.w).max(self.x - r.x - r.w);
        }
        let p1: [Point; 4] = [
            Point::new(self.x, self.y),
            Point::new(self.x + self.w, self.y),
            Point::new(self.x + self.w, self.y + self.h),
            Point::new(self.x, self.y + self.h),
        ];
        let p2: [Point; 4] = [
            Point::new(r.x, r.y),
            Point::new(r.x + r.w, r.y),
            Point::new(r.x + r.w, r.y + r.h),
            Point::new(r.x, r.y + r.h),
        ];
        let mut ret = 99999f32;
        for i in p1.iter() {
            for j in p2.iter() {
                ret = ret.min(i.distance(j))
            }
        }
        ret
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    pub fn distance(&self, &p: &Point) -> f32 {
        unsafe {
            sdl3_sys::stdinc::SDL_sqrtf(
                (self.x - p.x) * (self.x - p.x) + (self.y - p.y) * (self.y - p.y),
            )
        }
    }
}
