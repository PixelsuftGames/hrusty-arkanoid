use crate::{app, col::Color, ldr, rect::Point, ren, scene_base::{Event, SceneBase}};

#[derive(Debug, Default)]
pub struct SceneMenu {
    space_timer: f32
}

impl SceneMenu {
    pub unsafe fn init(&mut self) {
        self.space_timer = 0f32;
    }

    pub unsafe fn update(&mut self, dt: f32) {
        self.space_timer += dt * 5f32;
        if self.space_timer >= core::f32::consts::PI * 2f32 {
            self.space_timer -= core::f32::consts::PI * 2f32;
        }
    }

    pub unsafe fn draw(&mut self) {
        ren::clear(&Color::new_rgb(0f32, 0f32, 0f32));
        ldr::get_tex(0).draw(&Point::new(0f32, -100f32));
        ldr::get_tex(5).draw(&Point::new(400f32 - 399f32 / 2f32, 150f32 - 93f32));
        ldr::get_tex(6).alpha((sdl3_sys::stdinc::SDL_sinf(self.space_timer) + 1f32) / 2f32);
        ldr::get_tex(6).draw(&Point::new(400f32 - 200f32, 450f32 - 60f32));
        ren::present();
    }

    pub unsafe fn event(&mut self, ev: Event) {
        match ev {
            Event::Space => {
                app::run_scene(SceneBase::new_game());
            },
            _ => {}
        }
    }
}
