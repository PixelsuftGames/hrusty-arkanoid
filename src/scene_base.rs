use crate::{scene_game, scene_menu};

pub enum Event {
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
    Space,
    C
}

pub enum SceneBase {
    Game(scene_game::SceneGame),
    Menu(scene_menu::SceneMenu)
}

impl SceneBase {
    pub unsafe fn new_game() -> SceneBase {
        let mut ret = SceneBase::Game(Default::default());
        ret.init();
        ret
    }

    pub unsafe fn new_menu() -> SceneBase {
        let mut ret = SceneBase::Menu(Default::default());
        ret.init();
        ret
    }

    pub unsafe fn init(&mut self) {
        match self {
            SceneBase::Game(sc) => sc.init(),
            SceneBase::Menu(sc) => sc.init(),
        }
    }

    pub unsafe fn update(&mut self, dt: f32) {
        match self {
            SceneBase::Game(sc) => sc.update(dt),
            SceneBase::Menu(sc) => sc.update(dt)
        }
    }

    pub unsafe fn draw(&mut self) {
        match self {
            SceneBase::Game(sc) => sc.draw(),
            SceneBase::Menu(sc) => sc.draw()
        }
    }

    pub unsafe fn event(&mut self, ev: Event) {
        match self {
            SceneBase::Game(sc) => sc.event(ev),
            SceneBase::Menu(sc) => sc.event(ev)
        }
    }
}
