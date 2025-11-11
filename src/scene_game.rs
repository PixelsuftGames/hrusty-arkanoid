use crate::{app, col::Color, ldr, rect::{Point, Rect}, ren, scene_base::{self, Event}};
const BRICK_COLS: i32 = 10;
const BRICK_ROWS: i32 = 5;
const TOTAL_BRICKS: i32 = BRICK_COLS * BRICK_ROWS + 4;
const MARGIN: f32 = 16f32;
const PADDING: f32 = (800f32 - MARGIN * 2f32 - 64f32 * BRICK_COLS as f32) / (BRICK_COLS as f32 - 1f32);
const DEF_SPEED: f32 = 200f32;
const MAX_DT: f32 = 1f32 / 15f32;

trait Entity {
    unsafe fn do_move(&mut self, dt: f32);
    unsafe fn draw(&mut self);
    unsafe fn sync(&mut self);
    unsafe fn collides(&mut self, ball: &Ball) -> bool;
    unsafe fn hit(&mut self, ball: &mut Ball);
}

#[derive(Debug, Default, Clone, Copy)]
struct BrickState {
    hp: i32
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Brick {
    cur_state: BrickState,
    new_state: BrickState,
    rect: Rect
}

impl Brick {
    pub unsafe fn is_dead(&self) -> bool {
        self.cur_state.hp == 0
    }
}

impl Entity for Brick {
    unsafe fn draw(&mut self) {
        if self.is_dead() || self.cur_state.hp == 999 {
            return;
        }
        ldr::get_tex(1).color(&match self.cur_state.hp {
            1 => Color::new_rgb255(255f32, 255f32, 255f32),
            2 => Color::new_rgb255(34f32, 177f32, 76f32),
            3 => Color::new_rgb255(255f32, 127f32, 39f32),
            4 => Color::new_rgb255(237f32, 28f32, 36f32),
            5 => Color::new_rgb255(0f32, 174f32, 250f32),
            _ => unreachable!()
        });
        ldr::get_tex(1).draw(&Point::new(self.rect.x, self.rect.y));
    }

    unsafe fn do_move(&mut self, _dt: f32) {
        self.new_state = self.cur_state;
    }

    unsafe fn sync(&mut self) {
        self.cur_state = self.new_state;
    }

    unsafe fn collides(&mut self, ball: &Ball) -> bool {
        self.cur_state.hp > 0 && ball.new_state.rect.intersects(&self.rect)
    }

    unsafe fn hit(&mut self, ball: &mut Ball) {
        if self.cur_state.hp == 0 {
            return;
        }
        if self.cur_state.hp != 999 {
            self.cur_state.hp -= 1;
        }
        let inter = ball.cur_state.rect.collision(&self.rect);
        if self.rect.y == 600f32 {
            ball.cur_state.velocity.y *= -1f32;
            ball.cur_state.hp -= 1;
            if ball.cur_state.hp == 0 {
                ball.cur_state.velocity.x = 0.0000001f32;
                ball.cur_state.velocity.y = 0.0000001f32;
                ball.cur_state.rect.x = 1000f32;
                ball.cur_state.rect.y = 1000f32;
            }
        }
        else if self.rect.w == 100f32 {
            ball.cur_state.velocity.x *= -1f32;
        }
        else if self.rect.h == 100f32 {
            ball.cur_state.velocity.y *= -1f32;
        }
        else if inter.w == inter.h {
            ball.cur_state.velocity.x *= -1f32;
            ball.cur_state.velocity.y *= -1f32;
        }
        else if inter.w > inter.h {
            ball.cur_state.velocity.y *= -1f32;
        }
        else {
            ball.cur_state.velocity.x *= -1f32;
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct PaddleState {
    rect: Rect,
    velocity: Point
}

#[derive(Debug, Default, Clone, Copy)]
struct Paddle {
    cur_state: PaddleState,
    new_state: PaddleState,
    holding: i32
}

impl Entity for Paddle {
    unsafe fn draw(&mut self) {
        ldr::get_tex(2).draw(&Point::new(self.cur_state.rect.x, self.cur_state.rect.y));
    }

    unsafe fn do_move(&mut self, dt: f32) {
        self.new_state = self.cur_state;
        // self.new_state.rect.y += self.cur_state.velocity.y * dt / 2f32;
        if self.holding == 0 {
            if self.new_state.velocity.x >= 0f32 {
                self.new_state.velocity.x -= 50f32 * dt;
            }
            else {
                self.new_state.velocity.x += 50f32 * dt;
            }
        }
        else if self.new_state.velocity.x.abs() < 350f32 {
            self.new_state.velocity.x += dt * self.holding as f32 * 500f32;
            // accurately clamp values
            if self.new_state.velocity.x.abs() > 350f32 {
                let extra_time = (self.new_state.velocity.x.abs() - 350f32) / dt / 500f32;
                self.new_state.velocity.x = 350f32 * self.new_state.velocity.x.signum();
                self.new_state.rect.x += self.cur_state.velocity.x * (dt - extra_time) / 2f32;
                self.new_state.rect.x += self.new_state.velocity.x * (dt - extra_time) / 2f32;
                self.new_state.rect.x += self.new_state.velocity.x * extra_time;
                self.new_state.rect.x = self.new_state.rect.x.min(800f32 - self.cur_state.rect.w).max(0f32);
                return;
            }
        }
        self.new_state.rect.x += self.cur_state.velocity.x * dt / 2f32;
        self.new_state.rect.x += self.new_state.velocity.x * dt / 2f32;
        self.new_state.rect.x = self.new_state.rect.x.min(800f32 - self.cur_state.rect.w).max(0f32);
        // self.new_state.rect.y += self.cur_state.velocity.y * dt / 2f32;
    }

    unsafe fn sync(&mut self) {
        self.cur_state = self.new_state;
    }

    unsafe fn collides(&mut self, ball: &Ball) -> bool {
        ball.new_state.rect.intersects(&self.new_state.rect)
    }

    unsafe fn hit(&mut self, ball: &mut Ball) {
        let inter = ball.cur_state.rect.collision(&self.cur_state.rect);
        if inter.w == inter.h {
            ball.cur_state.velocity.x *= -1f32;
            ball.cur_state.velocity.y *= -1f32;
        }
        else if inter.w > inter.h {
            ball.cur_state.velocity.y *= -1f32;
        }
        else {
            ball.cur_state.velocity.x *= -1f32;
        }
        ball.cur_state.velocity.x += self.cur_state.velocity.x / 7f32;
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct BallState {
    rect: Rect,
    velocity: Point,
    hp: i32
}

#[derive(Debug, Default, Clone, Copy)]
struct Ball {
    cur_state: BallState,
    new_state: BallState
}

impl Ball {
    pub unsafe fn do_move(&mut self, dt: f32) {
        self.new_state = self.cur_state;
        self.new_state.rect.x += self.cur_state.velocity.x * dt;
        self.new_state.rect.y += self.cur_state.velocity.y * dt;
    }

    pub unsafe fn draw(&mut self) {
        ldr::get_tex(3).draw(&Point::new(self.cur_state.rect.x, self.cur_state.rect.y));
    }

    pub unsafe fn sync(&mut self) {
        self.cur_state = self.new_state;
    }
}

pub struct SceneGame {
    bricks: [Brick; TOTAL_BRICKS as usize],
    paddle: Paddle,
    ball: Ball
}

impl SceneGame {
    pub unsafe fn init(&mut self) {
        let mut cur_y = 52f32;
        let mut hp = 5;
        for y in 0..BRICK_ROWS {
            let mut cur_x = MARGIN;
            for x in 0..BRICK_COLS {
                let br = &mut self.bricks[(y * BRICK_COLS + x) as usize];
                br.rect = Rect::new(cur_x, cur_y, 64f32, 32f32);
                br.cur_state.hp = hp;
                cur_x += 64f32 + PADDING;
            }
            cur_y += 32f32 + PADDING;
            hp -= 1;
        }
        self.bricks[(BRICK_COLS * BRICK_ROWS) as usize].cur_state.hp = 999;
        self.bricks[(BRICK_COLS * BRICK_ROWS + 1) as usize].cur_state.hp = 999;
        self.bricks[(BRICK_COLS * BRICK_ROWS + 2) as usize].cur_state.hp = 999;
        self.bricks[(BRICK_COLS * BRICK_ROWS + 3) as usize].cur_state.hp = 999;
        // top
        self.bricks[(BRICK_COLS * BRICK_ROWS) as usize].rect = Rect::new(-100f32, -100f32, 1000f32, 100f32);
        // left
        self.bricks[(BRICK_COLS * BRICK_ROWS + 1) as usize].rect = Rect::new(-100f32, -100f32, 100f32, 800f32);
        // right
        self.bricks[(BRICK_COLS * BRICK_ROWS + 2) as usize].rect = Rect::new(800f32, -100f32, 100f32, 800f32);
        // bottom (deadly)
        self.bricks[(BRICK_COLS * BRICK_ROWS + 3) as usize].rect = Rect::new(-100f32, 600f32, 1000f32, 100f32);
        self.reset_attempt();
        self.ball.cur_state.hp = 3;
    }

    pub unsafe fn update(&mut self, orig_dt: f32) {
        let mut dt = orig_dt;
        while dt > MAX_DT {
            self.update(MAX_DT);
            dt -= MAX_DT;
        }
        let mut done = true;
        let mut dt_left = dt;
        let prev_hp = self.ball.cur_state.hp;
        while dt_left > 0f32 {
            let mut min_col_t = dt_left;
            // Dummy value
            let mut col_obj: *mut dyn Entity = &mut self.paddle as *mut dyn Entity;
            self.ball.do_move(dt_left);
            self.paddle.do_move(dt_left);
            if self.paddle.collides(&self.ball) {
                min_col_t = SceneGame::find_collision_time(&mut self.ball, &mut self.paddle, dt_left);
                col_obj = &mut self.paddle;
            }
            for brick in self.bricks.iter_mut() {
                done &= brick.is_dead();
                brick.do_move(dt_left);
                if brick.collides(&self.ball) {
                    let col_t = SceneGame::find_collision_time(&mut self.ball, brick, dt_left);
                    if col_t < min_col_t {
                        min_col_t = col_t;
                        col_obj = brick;
                    }
                }
            }
            if min_col_t < dt_left && !done {
                let col_obj = &mut *col_obj;
                self.ball.do_move(min_col_t);
                self.ball.sync();
                self.paddle.do_move(min_col_t);
                self.paddle.sync();
                for brick in self.bricks.iter_mut() {
                    brick.do_move(min_col_t);
                    brick.sync();
                }
                col_obj.hit(&mut self.ball);
                dt_left -= min_col_t;
            }
            else {
                break;
            }
        }
        if done {
            app::run_scene(crate::scene_base::SceneBase::new_menu());
            return;
        }
        self.ball.sync();
        self.paddle.sync();
        for brick in self.bricks.iter_mut() {
            brick.sync();
        }
        if prev_hp != self.ball.cur_state.hp {
            if self.ball.cur_state.hp == 0 {
                app::run_scene(scene_base::SceneBase::new_menu());
            }
            else {
                self.reset_attempt();
            }
        }
    }

    unsafe fn find_collision_time(ball: &mut Ball, obj: &mut dyn Entity, dt: f32) -> f32 {
        let mut left = 0f32;
        ball.do_move(0f32);
        obj.do_move(0f32);
        // Softlock check
        if obj.collides(&ball) {
            // panic!("Softlock");
            ball.do_move(dt);
            obj.do_move(dt);
            return dt;
        }
        let mut right = dt;
        while (right - left) > core::f32::EPSILON {
            let center = (left + right) / 2f32;
            ball.do_move(center);
            obj.do_move(center);
            if obj.collides(&ball) {
                right = center;
            }
            else {
                left = center;
            }
        }
        // Go back
        ball.do_move(dt);
        obj.do_move(dt);
        // For sure
        right
    }

    pub unsafe fn draw(&mut self) {
        ren::clear(&Color::new_rgb(0f32, 0f32, 50f32 / 255f32));
        ldr::get_tex(0).draw(&Point::new(0f32, -100f32));
        for brick in self.bricks.iter_mut() {
            brick.draw()
        }
        self.paddle.draw();
        for i in 0..self.ball.cur_state.hp {
            ldr::get_tex(4).draw(&Point::new(MARGIN + (i * 34) as f32, 10f32));
        }        
        self.ball.draw();
        ren::present();
    }

    pub unsafe fn reset_attempt(&mut self) {
        self.paddle.cur_state.rect = Rect::new(368f32, 558f32, 64f32, 32f32);
        self.paddle.cur_state.velocity = Point::default();
        self.ball.cur_state.rect = Rect::new(388f32, 524f32, 24f32, 24f32);
        self.ball.cur_state.velocity = Point::default();
    }

    pub unsafe fn event(&mut self, ev: Event) {
        match ev {
            Event::Space => {
                if self.ball.cur_state.velocity.x == 0f32 && self.ball.cur_state.velocity.y == 0f32 {
                    self.ball.cur_state.velocity.x = if sdl3_sys::stdinc::SDL_rand(2) == 1 { DEF_SPEED } else { -DEF_SPEED };
                    self.ball.cur_state.velocity.y = DEF_SPEED;
                }
            },
            Event::LeftDown | Event::RightUp => {
                self.paddle.holding -= 1;
            },
            Event::LeftUp | Event::RightDown => {
                self.paddle.holding += 1;
            },
            Event::C => {
                // Cheat
                self.ball.cur_state.hp = 20;
            }
        }
    }
}

impl Default for SceneGame {
    fn default() -> SceneGame {
        SceneGame {
            bricks: [Default::default(); TOTAL_BRICKS as usize],
            paddle: Default::default(),
            ball: Default::default()
        }
    }
}
