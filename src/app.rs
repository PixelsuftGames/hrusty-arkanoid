use crate::{
    audio, clock, cs, hrust, ldr, ren,
    scene_base::{self, Event, SceneBase},
    win,
};
use sdl3_sys::{events, scancode, stdinc, timer};

pub struct App {
    clock: clock::Clock,
    scene: scene_base::SceneBase,
    loader: ldr::Loader,
    running: bool,
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
    a().scene.update(a().clock.dt);
}

pub unsafe fn draw() {
    a().scene.draw();
}

pub unsafe fn run() {
    a().running = true;
    win::set_title(cs!("Arkanoid"));
    audio::music_play();
    win::set_shown(true);
    sdl3_sys::mouse::SDL_HideCursor();
    a().clock.reset();
    while a().running {
        let mut ev: events::SDL_Event = events::SDL_Event::default();
        while events::SDL_PollEvent(&mut ev as *mut events::SDL_Event) {
            match events::SDL_EventType(ev.r#type) {
                events::SDL_EVENT_QUIT => {
                    a().running = false;
                }
                events::SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED => {
                    ren::update_scale();
                }
                events::SDL_EVENT_KEY_DOWN | events::SDL_EVENT_KEY_UP => {
                    if ev.key.down && ev.key.repeat {
                        break;
                    }
                    match ev.key.scancode {
                        scancode::SDL_SCANCODE_A | scancode::SDL_SCANCODE_LEFT => {
                            // Move
                            a().scene.event(if ev.key.down {
                                Event::LeftDown
                            } else {
                                Event::LeftUp
                            });
                        }
                        scancode::SDL_SCANCODE_D | scancode::SDL_SCANCODE_RIGHT => {
                            // Move
                            a().scene.event(if ev.key.down {
                                Event::RightDown
                            } else {
                                Event::RightUp
                            });
                        }
                        scancode::SDL_SCANCODE_C => {
                            // Cheat
                            if ev.key.down {
                                a().scene.event(Event::C);
                            }
                        }
                        scancode::SDL_SCANCODE_R => {
                            // Reset
                            if ev.key.down {
                                a().scene.init();
                            }
                        }
                        scancode::SDL_SCANCODE_SPACE
                        | scancode::SDL_SCANCODE_RETURN
                        | scancode::SDL_SCANCODE_RETURN2 => {
                            if ev.key.down {
                                a().scene.event(Event::Space);
                            }
                        }
                        scancode::SDL_SCANCODE_V => {
                            // Toggle VSync
                            if ev.key.down {
                                ren::toggle_vsync();
                            }
                        }
                        scancode::SDL_SCANCODE_ESCAPE => {
                            // Go back
                            if ev.key.down {
                                match &a().scene {
                                    SceneBase::Menu(_) => {
                                        a().running = false;
                                    }
                                    SceneBase::Game(_) => {
                                        run_scene(scene_base::SceneBase::new_menu());
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        let kb_state = sdl3_sys::keyboard::SDL_GetModState();
        if kb_state & sdl3_sys::keycode::SDL_KMOD_CTRL > 0 {
            // Emulate low fps
            timer::SDL_DelayNS(50000000);
        }
        if kb_state & sdl3_sys::keycode::SDL_KMOD_SHIFT > 0 {
            // Set title FPS
            let mut fps_buf = [0i8; 16];
            fps_buf[0] = 'F' as i8;
            fps_buf[1] = 'P' as i8;
            fps_buf[2] = 'S' as i8;
            fps_buf[3] = ':' as i8;
            fps_buf[4] = ' ' as i8;
            stdinc::SDL_itoa(
                a().clock.get_fps(),
                fps_buf.as_mut_ptr().wrapping_add(5),
                10,
            );
            win::set_title(fps_buf.as_ptr());
        } else {
            win::set_title(cs!("Arkanoid"));
        }
        update();
        draw();
    }
}
