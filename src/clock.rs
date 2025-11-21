use sdl3_sys::timer;

#[derive(Clone, Copy, Debug)]
pub enum ClockType {
    TM, // Default Timer
    PC, // Performance Counter
}

pub struct Clock {
    pub dt: f32, // Delta time
    freq: f64,
    last_tick: u64,
    double_dt: f64,
    tp: ClockType,
}

impl Clock {
    pub unsafe fn new(tp: ClockType) -> Clock {
        Clock {
            dt: 0f32,
            freq: 0f64,
            last_tick: 0u64,
            double_dt: 0f64,
            tp: tp,
        }
    }

    pub unsafe fn reset(&mut self) {
        match self.tp {
            ClockType::TM => {
                self.freq = 1000f64;
                self.last_tick = timer::SDL_GetTicks();
            }
            ClockType::PC => {
                self.freq = timer::SDL_GetPerformanceFrequency() as f64;
                self.last_tick = timer::SDL_GetPerformanceCounter();
            }
        }
    }

    pub unsafe fn update(&mut self) {
        let now = match self.tp {
            ClockType::TM => timer::SDL_GetTicks(),
            ClockType::PC => timer::SDL_GetPerformanceCounter(),
        };
        self.double_dt = (now - self.last_tick) as f64 / self.freq;
        self.last_tick = now;
        self.dt = self.double_dt as f32;
    }

    pub unsafe fn get_fps(&mut self) -> i32 {
        let ret = 1f64 / self.double_dt;
        if ret > self.freq {
            return self.freq as i32;
        }
        ret as i32
    }
}
