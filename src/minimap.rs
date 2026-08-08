use raylib::prelude::*;

use crate::{
    player::Player,
    raycaster::RayHit,
};

pub fn draw_minimap(
    draw: &mut RaylibDrawHandle,
    width: i32,
    map: &[Vec<char>],
    player: &Player,
    rays: &[RayHit],
) {
    let scale =
        2.45_f32;

    let map_width =
        map[0].len()
            as f32
            * scale;

    let map_height =
        map.len()
            as f32
            * scale;

    let origin_x =
        width as f32
        - map_width
        - 18.0;

    let origin_y =
        18.0;

    draw.draw_rectangle(
        origin_x as i32 - 6,
        origin_y as i32 - 6,

        map_width as i32
            + 12,

        map_height as i32
            + 12,

        Color::new(
            10,
            18,
            12,
            220,
        ),
    );

    for (row, line)
        in map.iter()
            .enumerate()
    {
        for (col, tile)
            in line.iter()
                .enumerate()
        {
            draw.draw_rectangle(
                (
                    origin_x
                    + col as f32
                        * scale
                ) as i32,

                (
                    origin_y
                    + row as f32
                        * scale
                ) as i32,

                scale.ceil()
                    as i32,

                scale.ceil()
                    as i32,

                tile_color(
                    *tile
                ),
            );
        }
    }

    let player_x =
        origin_x
        + player.x
            * scale;

    let player_y =
        origin_y
        + player.y
            * scale;

    for ray
        in rays.iter()
            .step_by(30)
    {
        draw.draw_line(
            player_x as i32,
            player_y as i32,

            (
                origin_x
                + ray.hit_x
                    * scale
            ) as i32,

            (
                origin_y
                + ray.hit_y
                    * scale
            ) as i32,

            Color::new(
                255,
                235,
                110,
                70,
            ),
        );
    }

    draw.draw_circle(
        player_x as i32,
        player_y as i32,
        4.0,
        Color::YELLOW,
    );

    draw.draw_line(
        player_x as i32,
        player_y as i32,

        (
            player_x
            + player.angle.cos()
                * 14.0
        ) as i32,

        (
            player_y
            + player.angle.sin()
                * 14.0
        ) as i32,

        Color::RED,
    );
}

fn tile_color(
    tile: char,
) -> Color {
    match tile {
        '#' =>
            Color::new(
                30,
                90,
                40,
                255,
            ),

        'H' =>
            Color::new(
                45,
                135,
                55,
                255,
            ),

        'S' =>
            Color::new(
                155,
                155,
                160,
                255,
            ),

        'W' =>
            Color::new(
                60,
                150,
                220,
                255,
            ),

        'P' =>
            Color::new(
                184,
                161,
                120,
                255,
            ),

        'M' =>
            Color::RAYWHITE,

        'F' =>
            Color::new(
                230,
                100,
                155,
                255,
            ),

        'T' =>
            Color::new(
                25,
                85,
                35,
                255,
            ),

        _ =>
            Color::new(
                80,
                150,
                80,
                255,
            ),
    }
}