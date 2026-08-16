use raylib::prelude::*;

use crate::{
    player::Player,
    race::Race,
    raycaster::RayHit,
};

pub fn draw_minimap(
    draw: &mut RaylibDrawHandle,
    width: i32,
    map: &[Vec<char>],
    player: &Player,
    race: &Race,
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
                55,
            ),
        );
    }

    // Checkpoint activo
    if let Some(checkpoint) =
        race.active_checkpoint()
    {
        let checkpoint_x =
            origin_x
            + checkpoint.x
                * scale;

        let checkpoint_y =
            origin_y
            + checkpoint.y
                * scale;

        // Halo exterior
        draw.draw_circle_lines(
            checkpoint_x as i32,
            checkpoint_y as i32,
            7.0,
            Color::YELLOW,
        );

        // Centro
        draw.draw_circle(
            checkpoint_x as i32,
            checkpoint_y as i32,
            3.0,
            Color::YELLOW,
        );

        // Línea desde jugador al objetivo
        draw.draw_line(
            player_x as i32,
            player_y as i32,
            checkpoint_x as i32,
            checkpoint_y as i32,
            Color::new(
                255,
                230,
                70,
                180,
            ),
        );
    }

    draw.draw_circle(
        player_x as i32,
        player_y as i32,
        4.0,
        Color::RAYWHITE,
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
                70,
                72,
                76,
                255,
            ),

        'K' =>
            Color::new(
                235,
                55,
                50,
                255,
            ),

        'L' =>
            Color::new(
                235,
                220,
                70,
                255,
            ),

        'M' =>
            Color::RAYWHITE,

        'R' =>
            Color::new(
                245,
                185,
                35,
                255,
            ),

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

        'O' =>
            Color::new(
                245,
                125,
                25,
                255,
            ),

        'C' =>
            Color::new(
                150,
                95,
                50,
                255,
            ),

        'B' =>
            Color::new(
                220,
                60,
                60,
                255,
            ),

        'A' =>
            Color::new(
                190,
                190,
                200,
                255,
            ),

        'G' =>
            Color::new(
                85,
                100,
                145,
                255,
            ),

        'N' =>
            Color::new(
                255,
                255,
                255,
                255,
            ),

        'Y' =>
            Color::new(
                245,
                215,
                55,
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
