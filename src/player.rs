use raylib::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub velocity: f32,
    pub steering: f32,

    max_speed: f32,
    reverse_speed: f32,
    acceleration: f32,
    friction: f32,
    rotation_speed: f32,
    low_speed_rotation: f32,
    radius: f32,
}

impl Player {
    pub fn new(
        x: f32,
        y: f32,
    ) -> Self {
        Self {
            x,
            y,
            angle: 0.0,
            velocity: 0.0,
            steering: 0.0,

            max_speed: 5.5,
            reverse_speed: 2.8,
            acceleration: 8.5,
            friction: 4.8,

            rotation_speed: 3.8,
            low_speed_rotation: 2.7,

            radius: 0.22,
        }
    }

    pub fn update(
        &mut self,
        rl: &RaylibHandle,
        map: &[Vec<char>],
        dt: f32,
    ) {
        self.update_speed(
            rl,
            dt,
        );

        self.update_rotation(
            rl,
            dt,
        );

        self.update_position(
            map,
            dt,
        );
    }

    fn update_speed(
        &mut self,
        rl: &RaylibHandle,
        dt: f32,
    ) {
        if rl.is_key_down(
            KeyboardKey::KEY_W
        )
            || rl.is_key_down(
                KeyboardKey::KEY_UP
            )
        {
            self.velocity +=
                self.acceleration
                    * dt;
        } else if rl.is_key_down(
            KeyboardKey::KEY_S
        )
            || rl.is_key_down(
                KeyboardKey::KEY_DOWN
            )
        {
            self.velocity -=
                self.acceleration
                    * dt;
        } else {
            if self.velocity > 0.0 {
                self.velocity -=
                    self.friction
                        * dt;

                if self.velocity < 0.0 {
                    self.velocity = 0.0;
                }
            } else if self.velocity < 0.0 {
                self.velocity +=
                    self.friction
                        * dt;

                if self.velocity > 0.0 {
                    self.velocity = 0.0;
                }
            }
        }

        self.velocity =
            self.velocity.clamp(
                -self.reverse_speed,
                self.max_speed,
            );
    }

    fn update_rotation(
        &mut self,
        rl: &RaylibHandle,
        dt: f32,
    ) {
        let mut turn_input =
            0.0;

        if rl.is_key_down(
            KeyboardKey::KEY_A
        )
            || rl.is_key_down(
                KeyboardKey::KEY_LEFT
            )
        {
            turn_input -= 1.0;
        }

        if rl.is_key_down(
            KeyboardKey::KEY_D
        )
            || rl.is_key_down(
                KeyboardKey::KEY_RIGHT
            )
        {
            turn_input += 1.0;
        }

        self.steering +=
            (
                turn_input
                - self.steering
            )
            * (14.0 * dt)
                .min(1.0);

        if turn_input == 0.0 {
            self.steering *=
                (1.0 - 10.0 * dt)
                    .max(0.0);

            return;
        }

        let speed =
            self.velocity.abs();

        let speed_ratio =
            (speed / self.max_speed)
                .clamp(0.0, 1.0);

        let turn_speed =
            if speed < 0.55 {
                self.low_speed_rotation
            } else {
                self.rotation_speed
                    * (
                        0.72
                        + speed_ratio
                            * 0.28
                    )
            };

        let reverse_direction =
            if self.velocity < -0.05 {
                -1.0
            } else {
                1.0
            };

        self.angle +=
            turn_input
                * turn_speed
                * reverse_direction
                * dt;

        let tau =
            std::f32::consts::TAU;

        while self.angle >= tau {
            self.angle -= tau;
        }

        while self.angle < 0.0 {
            self.angle += tau;
        }
    }

    fn update_position(
        &mut self,
        map: &[Vec<char>],
        dt: f32,
    ) {
        let movement =
            self.velocity
                * dt;

        let next_x =
            self.x
            + self.angle.cos()
                * movement;

        let next_y =
            self.y
            + self.angle.sin()
                * movement;

        let mut collision =
            false;

        if !self.collides(
            map,
            next_x,
            self.y,
        ) {
            self.x = next_x;
        } else {
            collision = true;
        }

        if !self.collides(
            map,
            self.x,
            next_y,
        ) {
            self.y = next_y;
        } else {
            collision = true;
        }

        if collision {
            self.velocity *=
                0.20;

            if self.velocity.abs()
                < 0.10
            {
                self.velocity = 0.0;
            }
        }
    }

    fn collides(
        &self,
        map: &[Vec<char>],
        x: f32,
        y: f32,
    ) -> bool {
        let r =
            self.radius;

        let points = [
            (x-r, y-r),
            (x,   y-r),
            (x+r, y-r),

            (x-r, y),
            (x+r, y),

            (x-r, y+r),
            (x,   y+r),
            (x+r, y+r),
        ];

        for (px, py)
            in points
        {
            if px < 0.0
                || py < 0.0
            {
                return true;
            }

            let col =
                px.floor()
                    as usize;

            let row =
                py.floor()
                    as usize;

            if row >= map.len()
                || col
                    >= map[row].len()
            {
                return true;
            }

            if is_solid(
                map[row][col]
            ) {
                return true;
            }
        }

        false
    }
}

pub fn is_solid(
    tile: char,
) -> bool {
    matches!(
        tile,
        '#'
        | 'H'
        | 'S'
        | 'W'
        | 'T'
    )
}

pub fn is_wall(
    tile: char,
) -> bool {
    matches!(
        tile,
        '#'
        | 'H'
        | 'S'
        | 'W'
    )
}
