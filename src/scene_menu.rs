use crate::{app, col::Color, ldr, rect::Point, ren, scene_base::{Event, SceneBase}};

#[derive(Debug, Default)]
pub struct SceneMenu {
    
}

impl SceneMenu {
    pub unsafe fn init(&mut self) {}

    pub unsafe fn update(&mut self, _dt: f32) {}

    pub unsafe fn draw(&mut self) {
        ren::clear(&Color::new_rgb(0f32, 0f32, 0f32));
        ldr::get_tex(0).draw(&Point::new(0f32, -100f32));
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
