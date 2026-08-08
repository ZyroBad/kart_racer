use raylib::prelude::*;

use crate::player::{is_wall, Player};

struct RayHit {
    distance: f32,
    corrected_distance: f32,
    tile: char,
    hit_x: f32,
    hit_y: f32,
}

pub struct Framebuffer {
    width: i32,
    height: i32,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn render(
        &self,
        draw: &mut RaylibDrawHandle,
        map: &[Vec<char>],
        player: &Player,
        fov: f32,
        number_of_rays: usize,
    ) {
        let rays = self.cast_all_rays(
            map,
            player,
            fov,
            number_of_rays,
        );

        self.draw_world(draw, &rays, fov);
        self.draw_minimap(draw, map, player, &rays);
        self.draw_kart(draw, player.velocity, player.steering);

        draw.draw_text(
            "W/S acelerar-reversa | A/D girar",
            18,
            18,
            21,
            Color::RAYWHITE,
        );

        draw.draw_text(
            "Garden Circuit",
            18,
            48,
            19,
            Color::RAYWHITE,
        );
    }

    fn draw_world(
        &self,
        draw: &mut RaylibDrawHandle,
        rays: &[RayHit],
        fov: f32,
    ) {
        let horizon = self.height / 2;

        // Cielo
        draw.draw_rectangle(
            0,
            0,
            self.width,
            horizon,
            Color::new(120, 190, 235, 255),
        );

        // Césped
        draw.draw_rectangle(
            0,
            horizon,
            self.width,
            self.height - horizon,
            Color::new(65, 125, 70, 255),
        );

        // Líneas simples para dar profundidad al suelo.
        for i in 1..10 {
            let t = i as f32 / 10.0;

            let y =
                horizon as f32
                + (t * t)
                    * (self.height - horizon) as f32;

            draw.draw_line(
                0,
                y as i32,
                self.width,
                y as i32,
                Color::new(75, 145, 80, 255),
            );
        }

        if rays.is_empty() {
            return;
        }

        let column_width =
            self.width as f32
                / rays.len() as f32;

        let projection_distance =
            (self.width as f32 / 2.0)
                / (fov / 2.0).tan();

        for (index, ray) in rays.iter().enumerate() {
            let base_wall_height =
                projection_distance
                    / ray.corrected_distance.max(0.05);

            // IMPORTANTE:
            // Cada tipo de objeto tiene una altura distinta.
            //
            // Antes todo valía 1.0 y por eso un seto cercano
            // llenaba prácticamente toda la pantalla.
            let height_factor =
                wall_height_factor(ray.tile);

            let wall_height =
                (base_wall_height * height_factor)
                    .clamp(
                        1.0,
                        self.height as f32 * 0.95,
                    );

            // Los objetos están apoyados en el suelo.
            //
            // El punto inferior se desplaza ligeramente
            // hacia abajo del horizonte según cercanía.
            let ground_offset =
                (90.0
                    / ray.corrected_distance.max(0.4))
                    .clamp(0.0, 150.0);

            let bottom =
                horizon as f32 + ground_offset;

            let top =
                bottom - wall_height;

            let x =
                index as f32 * column_width;

            let base =
                wall_color(ray.tile);

            let shade =
                (1.0
                    / (1.0 + ray.distance * 0.08))
                .clamp(0.42, 1.0);

            let color =
                Color::new(
                    (base.r as f32 * shade) as u8,
                    (base.g as f32 * shade) as u8,
                    (base.b as f32 * shade) as u8,
                    255,
                );

            draw.draw_rectangle(
                x as i32,
                top as i32,
                column_width.ceil() as i32 + 1,
                wall_height as i32,
                color,
            );
        }
    }

    fn cast_all_rays(
        &self,
        map: &[Vec<char>],
        player: &Player,
        fov: f32,
        number_of_rays: usize,
    ) -> Vec<RayHit> {
        let mut rays =
            Vec::with_capacity(number_of_rays);

        if number_of_rays == 0 {
            return rays;
        }

        let first_angle =
            player.angle - fov / 2.0;

        let step =
            if number_of_rays > 1 {
                fov / (number_of_rays - 1) as f32
            } else {
                0.0
            };

        for index in 0..number_of_rays {
            let ray_angle =
                first_angle
                    + index as f32 * step;

            let mut hit =
                self.cast_ray(
                    map,
                    player.x,
                    player.y,
                    ray_angle,
                );

            hit.corrected_distance =
                hit.distance
                    * (ray_angle - player.angle).cos();

            rays.push(hit);
        }

        rays
    }

    fn cast_ray(
        &self,
        map: &[Vec<char>],
        start_x: f32,
        start_y: f32,
        angle: f32,
    ) -> RayHit {
        let dx = angle.cos();
        let dy = angle.sin();

        let mut distance = 0.0;
        let mut hit_x = start_x;
        let mut hit_y = start_y;
        let mut tile = '#';

        while distance < 90.0 {
            let x =
                start_x + dx * distance;

            let y =
                start_y + dy * distance;

            if x < 0.0 || y < 0.0 {
                break;
            }

            let map_x =
                x.floor() as usize;

            let map_y =
                y.floor() as usize;

            if map_y >= map.len()
                || map_x >= map[map_y].len()
            {
                break;
            }

            hit_x = x;
            hit_y = y;

            let current =
                map[map_y][map_x];

            if is_wall(current) {
                tile = current;
                break;
            }

            distance += 0.025;
        }

        RayHit {
            distance,
            corrected_distance: distance,
            tile,
            hit_x,
            hit_y,
        }
    }

    fn draw_minimap(
        &self,
        draw: &mut RaylibDrawHandle,
        map: &[Vec<char>],
        player: &Player,
        rays: &[RayHit],
    ) {
        let scale = 2.45_f32;

        let map_width =
            map[0].len() as f32 * scale;

        let map_height =
            map.len() as f32 * scale;

        let origin_x =
            self.width as f32
                - map_width
                - 18.0;

        let origin_y = 18.0;

        draw.draw_rectangle(
            origin_x as i32 - 6,
            origin_y as i32 - 6,
            map_width as i32 + 12,
            map_height as i32 + 12,
            Color::new(10, 18, 12, 220),
        );

        for (row, line)
            in map.iter().enumerate()
        {
            for (col, tile)
                in line.iter().enumerate()
            {
                let color =
                    match tile {
                        '#' =>
                            Color::new(
                                30, 90, 40, 255
                            ),

                        'H' =>
                            Color::new(
                                45, 135, 55, 255
                            ),

                        'S' =>
                            Color::new(
                                155, 155, 160, 255
                            ),

                        'W' =>
                            Color::new(
                                60, 150, 220, 255
                            ),

                        'F' =>
                            Color::new(
                                230, 100, 155, 255
                            ),

                        _ =>
                            Color::new(
                                80, 150, 80, 255
                            ),
                    };

                draw.draw_rectangle(
                    (
                        origin_x
                            + col as f32 * scale
                    ) as i32,
                    (
                        origin_y
                            + row as f32 * scale
                    ) as i32,
                    scale.ceil() as i32,
                    scale.ceil() as i32,
                    color,
                );
            }
        }

        let px =
            origin_x + player.x * scale;

        let py =
            origin_y + player.y * scale;

        for ray in rays.iter().step_by(30) {
            draw.draw_line(
                px as i32,
                py as i32,
                (
                    origin_x
                        + ray.hit_x * scale
                ) as i32,
                (
                    origin_y
                        + ray.hit_y * scale
                ) as i32,
                Color::new(
                    255, 235, 110, 75
                ),
            );
        }

        draw.draw_circle(
            px as i32,
            py as i32,
            4.0,
            Color::YELLOW,
        );

        draw.draw_line(
            px as i32,
            py as i32,
            (
                px
                    + player.angle.cos() * 14.0
            ) as i32,
            (
                py
                    + player.angle.sin() * 14.0
            ) as i32,
            Color::RED,
        );
    }

    fn draw_kart(
        &self,
        draw: &mut RaylibDrawHandle,
        velocity: f32,
        steering: f32,
    ) {
        let center_x =
            self.width / 2;

        let bottom =
            self.height - 20;

        let bounce =
            (velocity.abs() * 1.5)
                .min(3.0) as i32;

        let turn_offset =
            (steering * 24.0) as i32;

        let body_shift =
            (steering * 10.0) as i32;

        let cx =
            center_x + turn_offset;

        let y =
            bottom - bounce;

        draw.draw_ellipse(
            cx,
            y - 8,
            85.0,
            18.0,
            Color::new(
                20, 20, 20, 150
            ),
        );

        draw.draw_rectangle(
            cx - 82 - body_shift,
            y - 69,
            28,
            55,
            Color::new(
                24, 25, 28, 255
            ),
        );

        draw.draw_rectangle(
            cx + 54 - body_shift,
            y - 69,
            28,
            55,
            Color::new(
                24, 25, 28, 255
            ),
        );

        draw.draw_rectangle(
            cx - 67,
            y - 42,
            134,
            24,
            Color::new(
                180, 35, 35, 255
            ),
        );

        draw.draw_rectangle(
            cx - 55 + body_shift,
            y - 90,
            110,
            50,
            Color::new(
                220, 43, 42, 255
            ),
        );

        draw.draw_rectangle(
            cx - 39 + body_shift,
            y - 111,
            78,
            27,
            Color::new(
                240, 57, 49, 255
            ),
        );

        draw.draw_rectangle(
            cx - 27 + body_shift,
            y - 122,
            54,
            35,
            Color::new(
                35, 38, 44, 255
            ),
        );

        draw.draw_circle(
            cx + body_shift,
            y - 136,
            24.0,
            Color::new(
                245, 185, 72, 255
            ),
        );

        draw.draw_rectangle(
            cx - 22 + body_shift,
            y - 153,
            44,
            18,
            Color::RED,
        );

        draw.draw_rectangle(
            cx - 14 + body_shift,
            y - 138,
            28,
            7,
            Color::SKYBLUE,
        );

        draw.draw_rectangle(
            cx - 19,
            y - 43,
            38,
            16,
            Color::RAYWHITE,
        );

        draw.draw_text(
            "RUST",
            cx - 16,
            y - 42,
            12,
            Color::BLACK,
        );
    }
}

// Altura visual de cada tipo de superficie.
//
// Esto es lo que evita que TODO parezca una pared
// enorme hasta el cielo.
fn wall_height_factor(tile: char) -> f32 {
    match tile {
        // Setos bajos.
        'H' => 0.42,

        // Límite exterior, un poco más alto.
        '#' => 0.60,

        // Piedra alrededor de la fuente.
        'S' => 0.32,

        // Agua debe verse muy baja.
        // Sigue bloqueando movimiento por ahora.
        'W' => 0.08,

        'R' | 'G' | 'Y' => 0.50,

        _ => 0.45,
    }
}

fn wall_color(tile: char) -> Color {
    match tile {
        '#' =>
            Color::new(
                40, 115, 50, 255
            ),

        'H' =>
            Color::new(
                55, 155, 65, 255
            ),

        'S' =>
            Color::new(
                180, 180, 190, 255
            ),

        'W' =>
            Color::new(
                55, 155, 225, 255
            ),

        'R' =>
            Color::new(
                220, 65, 65, 255
            ),

        'G' =>
            Color::new(
                65, 200, 95, 255
            ),

        'Y' =>
            Color::new(
                235, 205, 60, 255
            ),

        _ =>
            Color::new(
                60, 125, 225, 255
            ),
    }
}