use raylib::prelude::*;

use crate::player::{
    is_wall,
    Player,
};

#[derive(Clone, Copy)]
pub struct RayHit {
    pub distance: f32,
    pub corrected_distance: f32,
    pub tile: char,
    pub hit_x: f32,
    pub hit_y: f32,
}

pub fn cast_all_rays(
    map: &[Vec<char>],
    player: &Player,
    fov: f32,
    number_of_rays: usize,
) -> Vec<RayHit> {
    let mut rays =
        Vec::with_capacity(
            number_of_rays
        );

    if number_of_rays == 0 {
        return rays;
    }

    let first_angle =
        player.angle
        - fov / 2.0;

    let angle_step =
        if number_of_rays > 1 {
            fov
            / (number_of_rays - 1)
                as f32
        } else {
            0.0
        };

    for index
        in 0..number_of_rays
    {
        let ray_angle =
            first_angle
            + index as f32
                * angle_step;

        let mut hit =
            cast_ray(
                map,
                player.x,
                player.y,
                ray_angle,
            );

        hit.corrected_distance =
            hit.distance
            * (
                ray_angle
                - player.angle
            ).cos();

        rays.push(hit);
    }

    rays
}

fn cast_ray(
    map: &[Vec<char>],
    start_x: f32,
    start_y: f32,
    angle: f32,
) -> RayHit {
    let dx =
        angle.cos();

    let dy =
        angle.sin();

    let mut distance =
        0.0;

    let mut hit_x =
        start_x;

    let mut hit_y =
        start_y;

    let mut tile =
        '#';

    while distance < 90.0 {
        let x =
            start_x
            + dx * distance;

        let y =
            start_y
            + dy * distance;

        if x < 0.0
            || y < 0.0
        {
            break;
        }

        let map_x =
            x.floor()
                as usize;

        let map_y =
            y.floor()
                as usize;

        if map_y >= map.len()
            || map_x
                >= map[map_y].len()
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
        corrected_distance:
            distance,
        tile,
        hit_x,
        hit_y,
    }
}

pub fn draw_floor(
    draw: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
    map: &[Vec<char>],
    player: &Player,
    fov: f32,
) {
    let horizon =
        height / 2;

    let block =
        4_i32;

    let left_angle =
        player.angle
        - fov / 2.0;

    let right_angle =
        player.angle
        + fov / 2.0;

    let left_x =
        left_angle.cos();

    let left_y =
        left_angle.sin();

    let right_x =
        right_angle.cos();

    let right_y =
        right_angle.sin();

    let mut screen_y =
        horizon + 1;

    while screen_y < height {
        let vertical_distance =
            screen_y
            - horizon;

        let row_distance =
            (
                height as f32
                * 0.55
            )
            / vertical_distance
                .max(1)
                as f32;

        let start_x =
            player.x
            + left_x
                * row_distance;

        let start_y =
            player.y
            + left_y
                * row_distance;

        let end_x =
            player.x
            + right_x
                * row_distance;

        let end_y =
            player.y
            + right_y
                * row_distance;

        let mut screen_x =
            0;

        while screen_x < width {
            let t =
                screen_x as f32
                / width as f32;

            let world_x =
                start_x
                + (
                    end_x
                    - start_x
                ) * t;

            let world_y =
                start_y
                + (
                    end_y
                    - start_y
                ) * t;

            draw.draw_rectangle(
                screen_x,
                screen_y,
                block,
                block,
                floor_color(
                    map,
                    world_x,
                    world_y,
                ),
            );

            screen_x += block;
        }

        screen_y += block;
    }
}

pub fn draw_walls(
    draw: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
    rays: &[RayHit],
    fov: f32,
) {
    if rays.is_empty() {
        return;
    }

    let horizon =
        height / 2;

    let column_width =
        width as f32
        / rays.len()
            as f32;

    let projection_distance =
        (width as f32 / 2.0)
        / (fov / 2.0).tan();

    for (index, ray)
        in rays.iter()
            .enumerate()
    {
        let base_height =
            projection_distance
            / ray.corrected_distance
                .max(0.05);

        let wall_height =
            (
                base_height
                * wall_height_factor(
                    ray.tile
                )
            )
            .clamp(
                1.0,
                height as f32
                    * 0.95,
            );

        let ground_offset =
            (
                90.0
                / ray.corrected_distance
                    .max(0.4)
            )
            .clamp(
                0.0,
                150.0,
            );

        let bottom =
            horizon as f32
            + ground_offset;

        let top =
            bottom
            - wall_height;

        let x =
            index as f32
            * column_width;

        let base_color =
            wall_color(
                ray.tile
            );

        let shade =
            (
                1.0
                / (
                    1.0
                    + ray.distance
                        * 0.08
                )
            )
            .clamp(
                0.42,
                1.0,
            );

        let color =
            shade_color(
                base_color,
                shade,
            );

        draw.draw_rectangle(
            x as i32,
            top as i32,
            column_width.ceil()
                as i32
                + 1,
            wall_height as i32,
            color,
        );
    }
}

fn floor_color(
    map: &[Vec<char>],
    x: f32,
    y: f32,
) -> Color {
    if x < 0.0
        || y < 0.0
    {
        return grass_color();
    }

    let map_x =
        x.floor()
            as usize;

    let map_y =
        y.floor()
            as usize;

    if map_y >= map.len()
        || map_x
            >= map[map_y].len()
    {
        return grass_color();
    }

    match map[map_y][map_x] {
        'P' =>
            Color::new(
                70,
                72,
                76,
                255,
            ),

        'K' => {
            if (
                map_x
                + map_y
            ) % 2 == 0
            {
                Color::RAYWHITE
            } else {
                Color::new(
                    220,
                    40,
                    38,
                    255,
                )
            }
        }

        'L' =>
            Color::new(
                235,
                220,
                70,
                255,
            ),

        'M' => {
            if (
                map_x
                + map_y
            ) % 2 == 0
            {
                Color::RAYWHITE
            } else {
                Color::new(
                    45,
                    45,
                    48,
                    255,
                )
            }
        }

        'R' => {
            if (
                map_x
                + map_y
            ) % 2 == 0
            {
                Color::new(
                    245,
                    195,
                    45,
                    255,
                )
            } else {
                Color::new(
                    220,
                    55,
                    40,
                    255,
                )
            }
        }

        'W' =>
            Color::new(
                55,
                145,
                220,
                255,
            ),

        'F' =>
            Color::new(
                73,
                135,
                68,
                255,
            ),

        _ =>
            grass_color(),
    }
}

fn grass_color() -> Color {
    Color::new(
        65,
        125,
        70,
        255,
    )
}

fn wall_height_factor(
    tile: char,
) -> f32 {
    match tile {
        'H' => 0.42,
        '#' => 0.60,
        'S' => 0.32,
        'W' => 0.08,
        _ => 0.45,
    }
}

fn wall_color(
    tile: char,
) -> Color {
    match tile {
        '#' =>
            Color::new(
                40,
                115,
                50,
                255,
            ),

        'H' =>
            Color::new(
                55,
                155,
                65,
                255,
            ),

        'S' =>
            Color::new(
                180,
                180,
                190,
                255,
            ),

        'W' =>
            Color::new(
                55,
                155,
                225,
                255,
            ),

        _ =>
            Color::new(
                60,
                125,
                225,
                255,
            ),
    }
}

fn shade_color(
    color: Color,
    factor: f32,
) -> Color {
    Color::new(
        (
            color.r as f32
            * factor
        ) as u8,

        (
            color.g as f32
            * factor
        ) as u8,

        (
            color.b as f32
            * factor
        ) as u8,

        color.a,
    )
}
