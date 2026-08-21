use raylib::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub velocity: f32,
    pub steering: f32,
    pub drift: f32,
    pub boost_flash: f32,

    boost_timer: f32,
    max_speed: f32,
    reverse_speed: f32,
    acceleration: f32,
    friction: f32,
    rotation_speed: f32,
    low_speed_rotation: f32,
    radius: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            angle: 0.0,
            velocity: 0.0,
            steering: 0.0,
            drift: 0.0,
            boost_flash: 0.0,
            boost_timer: 0.0,

            max_speed: 5.55,
            reverse_speed: 2.45,
            acceleration: 8.6,
            friction: 4.75,

            rotation_speed: 3.2,
            low_speed_rotation: 2.35,

            radius: 0.17,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, map: &[Vec<char>], dt: f32, mouse_delta_x: f32) {
        let current_tile = tile_at(map, self.x, self.y);

        let surface_factor = surface_speed_factor(current_tile);

        self.update_speed(rl, dt, surface_factor, current_tile);

        self.update_rotation(rl, dt, mouse_delta_x);

        self.update_position(map, dt);

        self.update_drift(rl, dt);

        self.boost_flash = (self.boost_flash - dt * 2.6).max(0.0);

        self.boost_timer = (self.boost_timer - dt).max(0.0);
    }

    fn update_speed(
        &mut self,
        rl: &RaylibHandle,
        dt: f32,
        surface_factor: f32,
        current_tile: char,
    ) {
        let surface_acceleration = self.acceleration * (0.72 + surface_factor * 0.28);

        if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) {
            self.velocity += surface_acceleration * dt;
        } else if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.velocity -= surface_acceleration * dt;
        } else {
            if self.velocity > 0.0 {
                self.velocity -= self.friction * dt;

                if self.velocity < 0.0 {
                    self.velocity = 0.0;
                }
            } else if self.velocity < 0.0 {
                self.velocity += self.friction * dt;

                if self.velocity > 0.0 {
                    self.velocity = 0.0;
                }
            }
        }

        let boost_multiplier = if self.boost_timer > 0.0 { 1.15 } else { 1.0 };

        let current_max_speed = self.max_speed * surface_factor * boost_multiplier;

        let current_reverse_speed = self.reverse_speed * surface_factor.max(0.65);

        self.velocity = self
            .velocity
            .clamp(-current_reverse_speed, current_max_speed);

        if current_tile == 'R' && self.velocity > 1.0 {
            self.boost_timer = 0.70;

            self.velocity = (self.velocity + 8.2 * dt).min(self.max_speed * 1.14);

            self.boost_flash = 1.0;
        }
    }

    fn update_rotation(&mut self, rl: &RaylibHandle, dt: f32, mouse_delta_x: f32) {
        let mut turn_input = 0.0;

        if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT) {
            turn_input -= 1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            turn_input += 1.0;
        }

        let handbrake = rl.is_key_down(KeyboardKey::KEY_SPACE);

        self.steering += (turn_input - self.steering) * (14.0 * dt).min(1.0);

        if turn_input == 0.0 {
            self.steering *= (1.0 - 10.0 * dt).max(0.0);
        }

        let speed = self.velocity.abs();

        let speed_ratio = (speed / self.max_speed).clamp(0.0, 1.0);

        let mut turn_speed = if speed < 0.55 {
            self.low_speed_rotation
        } else {
            self.rotation_speed * (0.72 + speed_ratio * 0.28)
        };

        if handbrake && speed > 1.4 {
            turn_speed *= 1.42;
            self.velocity *= (1.0 - 0.55 * dt).max(0.0);
        }

        let reverse_direction = if self.velocity < -0.05 { -1.0 } else { 1.0 };

        self.angle += (turn_input * turn_speed * reverse_direction * dt) + mouse_delta_x * 0.0022;

        let tau = std::f32::consts::TAU;

        while self.angle >= tau {
            self.angle -= tau;
        }

        while self.angle < 0.0 {
            self.angle += tau;
        }
    }

    fn update_drift(&mut self, rl: &RaylibHandle, dt: f32) {
        let handbrake = rl.is_key_down(KeyboardKey::KEY_SPACE);

        let target = (self.steering.abs()
            * (self.velocity.abs() / self.max_speed)
            * if handbrake { 1.55 } else { 1.0 })
        .clamp(0.0, 1.0);

        self.drift += (target - self.drift) * (8.0 * dt).min(1.0);
    }

    fn update_position(&mut self, map: &[Vec<char>], dt: f32) {
        let movement = self.velocity * dt;

        let next_x = self.x + self.angle.cos() * movement;

        let next_y = self.y + self.angle.sin() * movement;

        let mut blocked_x = false;

        let mut blocked_y = false;

        if !self.collides(map, next_x, self.y) {
            self.x = next_x;
        } else {
            blocked_x = true;
        }

        if !self.collides(map, self.x, next_y) {
            self.y = next_y;
        } else {
            blocked_y = true;
        }

        if blocked_x && blocked_y {
            self.velocity *= 0.35;
        } else if blocked_x || blocked_y {
            self.velocity *= 0.82;
        }

        if (blocked_x || blocked_y) && self.velocity.abs() < 0.05 {
            self.velocity = 0.0;
        }
    }

    fn collides(&self, map: &[Vec<char>], x: f32, y: f32) -> bool {
        let r = self.radius;

        let points = [
            (x - r, y - r),
            (x, y - r),
            (x + r, y - r),
            (x - r, y),
            (x + r, y),
            (x - r, y + r),
            (x, y + r),
            (x + r, y + r),
        ];

        for (px, py) in points {
            if px < 0.0 || py < 0.0 {
                return true;
            }

            let col = px.floor() as usize;

            let row = py.floor() as usize;

            if row >= map.len() || col >= map[row].len() {
                return true;
            }

            if is_solid(map[row][col]) {
                return true;
            }
        }

        false
    }
}

pub fn is_solid(tile: char) -> bool {
    matches!(tile, '#' | 'H' | 'S' | 'W' | 'D')
}

pub fn is_wall(tile: char) -> bool {
    matches!(tile, '#' | 'H' | 'S' | 'W' | 'D')
}

fn tile_at(map: &[Vec<char>], x: f32, y: f32) -> char {
    if x < 0.0 || y < 0.0 {
        return '#';
    }

    let col = x.floor() as usize;

    let row = y.floor() as usize;

    if row >= map.len() || col >= map[row].len() {
        '#'
    } else {
        map[row][col]
    }
}

fn surface_speed_factor(tile: char) -> f32 {
    match tile {
        'P' | 'M' | 'K' | 'L' => 1.0,
        'R' => 1.08,
        'F' => 0.74,
        'U' => 0.70,
        '.' => 0.62,
        _ => 0.82,
    }
}
