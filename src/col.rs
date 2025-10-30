#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub fn new_rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r: r, g: g, b: b, a: 1f32 }
    }

    pub fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r: r, g: g, b: b, a: a }
    }

    pub fn new_rgb255(r: f32, g: f32, b: f32) -> Color {
        Color { r: r / 255f32, g: g / 255f32, b: b / 255f32, a: 1f32 }
    }

    pub fn new_rgba255(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r: r / 255f32, g: g / 255f32, b: b / 255f32, a: a / 255f32 }
    }
}
