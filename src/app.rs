use crate::{clock, hrust, ldr, ren, scene_base::{self, Event, SceneBase}, win};
use sdl3_sys::{events, scancode};

pub struct App {
    clock: clock::Clock,
    scene: scene_base::SceneBase,
    loader: ldr::Loader,
    running: bool
}

static mut handle: *mut App = core::ptr::null_mut();

pub unsafe fn a() -> &'static mut App {
    &mut *handle
}

pub unsafe fn init() {
    handle = hrust::alloc_ptr();
    a().running = false;
    a().clock = clock::Clock::new(clock::ClockType::PC);
    ldr::init(&mut a().loader);
    run_scene(scene_base::SceneBase::new_menu());
}

pub unsafe fn run_scene(scene: scene_base::SceneBase) {
    a().scene = scene;
}

pub unsafe fn destroy() {
    ldr::destroy();
    hrust::free_ptr(handle);
}

pub unsafe fn update() {
    a().clock.update();
    // info!("FPS: %i", a().clock.get_fps());
    a().scene.update(a().clock.dt);
}

pub unsafe fn draw() {
    a().scene.draw();
}

pub unsafe fn run() {
    a().running = true;
    win::set_shown(true);
    a().clock.reset();
    while a().running {
        let mut ev: events::SDL_Event = events::SDL_Event::default();
        while events::SDL_PollEvent(&mut ev as *mut events::SDL_Event) {
            match events::SDL_EventType(ev.r#type) {
                events::SDL_EVENT_QUIT => {
                    a().running = false;
                },
                events::SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED => {
                    ren::update_scale();
                },
                events::SDL_EVENT_KEY_DOWN | events::SDL_EVENT_KEY_UP => {
                    if ev.key.scancode == scancode::SDL_SCANCODE_ESCAPE {
                        match &a().scene {
                            SceneBase::Menu(_) => {
                                a().running = false;
                            },
                            SceneBase::Game(_) => {
                                run_scene(scene_base::SceneBase::new_menu());
                            }
                        }
                        break;
                    }
                    if ev.key.down && ev.key.repeat {
                        break;
                    }
                    match ev.key.scancode {
                        scancode::SDL_SCANCODE_A | scancode::SDL_SCANCODE_LEFT => {
                            a().scene.event(if ev.key.down { Event::LeftDown } else { Event::LeftUp });
                        },
                        scancode::SDL_SCANCODE_D | scancode::SDL_SCANCODE_RIGHT => {
                            a().scene.event(if ev.key.down { Event::RightDown } else { Event::RightUp });
                        },
                        scancode::SDL_SCANCODE_SPACE => {
                            if ev.key.down {
                                a().scene.event(Event::Space);
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        update();
        draw();
    }    
}
