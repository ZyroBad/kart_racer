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
        self.draw_kart(draw, player.velocity);
        self.draw_controls(draw);
    }

    fn draw_world(
        &self,
        draw: &mut RaylibDrawHandle,
        rays: &[RayHit],
        fov: f32,
    ) {
        // Cielo.
        draw.draw_rectangle(
            0,
            0,
            self.width,
            self.height / 2,
            Color::new(78, 160, 225, 255),
        );

        // Línea de horizonte.
        draw.draw_rectangle(
            0,
            self.height / 2 - 4,
            self.width,
            8,
            Color::new(210, 225, 235, 255),
        );

        // Piso.
        draw.draw_rectangle(
            0,
            self.height / 2,
            self.width,
            self.height / 2,
            Color::new(55, 54, 58, 255),
        );

        // Bandas del piso para dar sensación de velocidad/profundidad.
        let horizon = self.height / 2;

        for i in 1..9 {
            let t = i as f32 / 9.0;
            let y = horizon as f32
                + (t * t) * (self.height - horizon) as f32;

            draw.draw_line(
                0,
                y as i32,
                self.width,
                y as i32,
                Color::new(68, 67, 72, 255),
            );
        }

        if rays.is_empty() {
            return;
        }

        let column_width =
            self.width as f32 / rays.len() as f32;

        let projection_plane_distance =
            (self.width as f32 / 2.0)
                / (fov / 2.0).tan();

        for (index, ray) in rays.iter().enumerate() {
            let wall_height =
                (projection_plane_distance
                    / ray.corrected_distance.max(0.01))
                .clamp(
                    1.0,
                    self.height as f32 * 1.6,
                );

            let center_y =
                self.height as f32 / 2.0;

            let top =
                center_y - wall_height / 2.0;

            let x =
                index as f32 * column_width;

            let base = wall_color(ray.tile);

            // Más lejos = más oscuro.
            let shade =
                (1.0 / (1.0 + ray.distance * 0.11))
                    .clamp(0.28, 1.0);

            let color = Color::new(
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
        let mut rays = Vec::with_capacity(number_of_rays);

        if number_of_rays == 0 {
            return rays;
        }

        let first_angle =
            player.angle - fov / 2.0;

        let angle_step = if number_of_rays > 1 {
            fov / (number_of_rays - 1) as f32
        } else {
            0.0
        };

        for index in 0..number_of_rays {
            let ray_angle =
                first_angle + index as f32 * angle_step;

            let mut hit = self.cast_ray(
                map,
                player.x,
                player.y,
                ray_angle,
            );

            // Corrección de fish-eye.
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
        let dir_x = angle.cos();
        let dir_y = angle.sin();

        let mut distance = 0.0;
        let step = 0.015;
        let max_distance = 40.0;

        let mut hit_x = start_x;
        let mut hit_y = start_y;
        let mut tile = '#';

        while distance < max_distance {
            let x =
                start_x + dir_x * distance;

            let y =
                start_y + dir_y * distance;

            if x < 0.0 || y < 0.0 {
                break;
            }

            let map_x = x.floor() as usize;
            let map_y = y.floor() as usize;

            if map_y >= map.len()
                || map_x >= map[map_y].len()
            {
                break;
            }

            hit_x = x;
            hit_y = y;

            let current_tile =
                map[map_y][map_x];

            if is_wall(current_tile) {
                tile = current_tile;
                break;
            }

            distance += step;
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
        let scale = 5.2_f32;

        let map_width =
            map[0].len() as f32 * scale;

        let map_height =
            map.len() as f32 * scale;

        let origin_x =
            self.width as f32 - map_width - 18.0;

        let origin_y = 18.0;

        draw.draw_rectangle(
            origin_x as i32 - 7,
            origin_y as i32 - 7,
            map_width as i32 + 14,
            map_height as i32 + 14,
            Color::new(10, 12, 16, 220),
        );

        draw.draw_rectangle_lines(
            origin_x as i32 - 7,
            origin_y as i32 - 7,
            map_width as i32 + 14,
            map_height as i32 + 14,
            Color::RAYWHITE,
        );

        for (row, line) in map.iter().enumerate() {
            for (col, tile) in line.iter().enumerate() {
                let color = match tile {
                    '#' => Color::new(55, 115, 220, 255),
                    'R' => Color::new(215, 65, 65, 255),
                    'G' => Color::new(65, 190, 90, 255),
                    'Y' => Color::new(230, 200, 55, 255),
                    _ => Color::new(28, 31, 38, 255),
                };

                draw.draw_rectangle(
                    (origin_x + col as f32 * scale) as i32,
                    (origin_y + row as f32 * scale) as i32,
                    scale.ceil() as i32,
                    scale.ceil() as i32,
                    color,
                );
            }
        }

        let px = origin_x + player.x * scale;
        let py = origin_y + player.y * scale;

        for ray in rays.iter().step_by(20) {
            let hx = origin_x + ray.hit_x * scale;
            let hy = origin_y + ray.hit_y * scale;

            draw.draw_line(
                px as i32,
                py as i32,
                hx as i32,
                hy as i32,
                Color::new(255, 220, 70, 100),
            );
        }

        draw.draw_circle(
            px as i32,
            py as i32,
            4.0,
            Color::GREEN,
        );

        draw.draw_line(
            px as i32,
            py as i32,
            (px + player.angle.cos() * 16.0) as i32,
            (py + player.angle.sin() * 16.0) as i32,
            Color::RED,
        );
    }

    fn draw_kart(
        &self,
        draw: &mut RaylibDrawHandle,
        velocity: f32,
    ) {
        let cx = self.width / 2;
        let base_y = self.height - 22;

        // Pequeño rebote visual al acelerar.
        let bounce =
            ((velocity.abs() * 2.0).sin() * 2.0) as i32;

        let y = base_y + bounce;

        // Sombra.
        draw.draw_ellipse(
            cx,
            y - 9,
            82.0,
            18.0,
            Color::new(20, 20, 20, 150),
        );

        // Ruedas traseras.
        draw.draw_rectangle(
            cx - 82,
            y - 69,
            28,
            55,
            Color::new(24, 25, 28, 255),
        );

        draw.draw_rectangle(
            cx + 54,
            y - 69,
            28,
            55,
            Color::new(24, 25, 28, 255),
        );

        // Detalle gris de las ruedas.
        draw.draw_rectangle(
            cx - 78,
            y - 58,
            20,
            31,
            Color::new(55, 58, 62, 255),
        );

        draw.draw_rectangle(
            cx + 58,
            y - 58,
            20,
            31,
            Color::new(55, 58, 62, 255),
        );

        // Parachoques trasero.
        draw.draw_rectangle(
            cx - 67,
            y - 42,
            134,
            24,
            Color::new(185, 38, 38, 255),
        );

        draw.draw_rectangle(
            cx - 57,
            y - 47,
            114,
            26,
            Color::new(225, 50, 45, 255),
        );

        // Chasis rojo, forma escalonada/pixel.
        draw.draw_rectangle(
            cx - 50,
            y - 92,
            100,
            50,
            Color::new(220, 43, 42, 255),
        );

        draw.draw_rectangle(
            cx - 38,
            y - 110,
            76,
            25,
            Color::new(240, 57, 49, 255),
        );

        // Asiento oscuro.
        draw.draw_rectangle(
            cx - 27,
            y - 119,
            54,
            32,
            Color::new(35, 38, 44, 255),
        );

        // Cabeza / casco del piloto.
        draw.draw_circle(
            cx,
            y - 134,
            24.0,
            Color::new(245, 185, 72, 255),
        );

        // Casco rojo.
        draw.draw_rectangle(
            cx - 22,
            y - 150,
            44,
            17,
            Color::new(220, 48, 43, 255),
        );

        draw.draw_rectangle(
            cx - 16,
            y - 157,
            32,
            9,
            Color::new(235, 62, 50, 255),
        );

        // Visera.
        draw.draw_rectangle(
            cx - 14,
            y - 137,
            28,
            7,
            Color::new(70, 125, 160, 255),
        );

        // Tubos de escape.
        draw.draw_rectangle(
            cx - 68,
            y - 82,
            14,
            29,
            Color::new(100, 105, 110, 255),
        );

        draw.draw_rectangle(
            cx + 54,
            y - 82,
            14,
            29,
            Color::new(100, 105, 110, 255),
        );

        // Luces traseras.
        draw.draw_rectangle(
            cx - 43,
            y - 60,
            18,
            12,
            Color::new(255, 190, 45, 255),
        );

        draw.draw_rectangle(
            cx + 25,
            y - 60,
            18,
            12,
            Color::new(255, 190, 45, 255),
        );

        // Placa.
        draw.draw_rectangle(
            cx - 18,
            y - 42,
            36,
            15,
            Color::new(235, 235, 220, 255),
        );

        draw.draw_text(
            "RUST",
            cx - 15,
            y - 41,
            12,
            Color::new(40, 40, 45, 255),
        );
    }

    fn draw_controls(
        &self,
        draw: &mut RaylibDrawHandle,
    ) {
        draw.draw_text(
            "W / ↑ acelerar   S / ↓ reversa   A/D o ←/→ girar",
            18,
            18,
            21,
            Color::RAYWHITE,
        );
    }
}

fn wall_color(tile: char) -> Color {
    match tile {
        'R' => Color::new(220, 65, 65, 255),
        'G' => Color::new(65, 200, 95, 255),
        'Y' => Color::new(235, 205, 60, 255),
        _ => Color::new(60, 125, 225, 255),
    }
}