use raylib::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub velocity: f32,

    max_speed: f32,
    reverse_speed: f32,
    acceleration: f32,
    friction: f32,
    rotation_speed: f32,
    radius: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            angle: 0.0,
            velocity: 0.0,

            max_speed: 5.0,
            reverse_speed: 2.5,
            acceleration: 8.0,
            friction: 5.0,
            rotation_speed: 2.4,
            radius: 0.20,
        }
    }

    pub fn update(
        &mut self,
        rl: &RaylibHandle,
        map: &[Vec<char>],
        delta_time: f32,
    ) {
        // Acelerar.
        if rl.is_key_down(KeyboardKey::KEY_W)
            || rl.is_key_down(KeyboardKey::KEY_UP)
        {
            self.velocity += self.acceleration * delta_time;
        }
        // Retroceder.
        else if rl.is_key_down(KeyboardKey::KEY_S)
            || rl.is_key_down(KeyboardKey::KEY_DOWN)
        {
            self.velocity -= self.acceleration * delta_time;
        }
        // Fricción cuando no se está acelerando.
        else {
            if self.velocity > 0.0 {
                self.velocity -= self.friction * delta_time;

                if self.velocity < 0.0 {
                    self.velocity = 0.0;
                }
            } else if self.velocity < 0.0 {
                self.velocity += self.friction * delta_time;

                if self.velocity > 0.0 {
                    self.velocity = 0.0;
                }
            }
        }

        self.velocity = self.velocity.clamp(
            -self.reverse_speed,
            self.max_speed,
        );

        // Girar.
        let mut turn = 0.0;

        if rl.is_key_down(KeyboardKey::KEY_A)
            || rl.is_key_down(KeyboardKey::KEY_LEFT)
        {
            turn -= 1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_D)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT)
        {
            turn += 1.0;
        }

        if turn != 0.0 {
            // Permite girar incluso detenido para que el avance
            // sea fácil de probar.
            self.angle += turn * self.rotation_speed * delta_time;
        }

        // Mantener ángulo en 0..TAU.
        let tau = std::f32::consts::TAU;

        while self.angle >= tau {
            self.angle -= tau;
        }

        while self.angle < 0.0 {
            self.angle += tau;
        }

        // Movimiento hacia donde mira el carro.
        let movement = self.velocity * delta_time;

        let next_x = self.x + self.angle.cos() * movement;
        let next_y = self.y + self.angle.sin() * movement;

        // Se prueban los ejes por separado para que el carro
        // pueda deslizarse contra una pared.
        if !self.collides(map, next_x, self.y) {
            self.x = next_x;
        } else {
            self.velocity *= 0.30;
        }

        if !self.collides(map, self.x, next_y) {
            self.y = next_y;
        } else {
            self.velocity *= 0.30;
        }
    }

    fn collides(
        &self,
        map: &[Vec<char>],
        x: f32,
        y: f32,
    ) -> bool {
        let points = [
            (x - self.radius, y - self.radius),
            (x + self.radius, y - self.radius),
            (x - self.radius, y + self.radius),
            (x + self.radius, y + self.radius),
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

            if is_wall(map[row][col]) {
                return true;
            }
        }

        false
    }
}

pub fn is_wall(tile: char) -> bool {
    matches!(tile, '#' | 'R' | 'G' | 'Y')
}