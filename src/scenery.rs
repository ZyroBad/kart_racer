use raylib::prelude::*;

use crate::{
    player::Player,
    race::Race,
    raycaster::RayHit,
};

#[derive(Clone, Copy)]
struct Scenery {
    x: f32,
    y: f32,
    kind: char,
}

pub fn draw_sky(
    draw: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
) {
    draw.draw_rectangle(
        0,
        0,
        width,
        height / 2,
        Color::new(
            120,
            190,
            235,
            255,
        ),
    );

    draw.draw_circle(
        width - 120,
        95,
        38.0,
        Color::new(
            250,
            220,
            95,
            255,
        ),
    );

    draw_cloud(
        draw,
        120,
        95,
        1.0,
    );

    draw_cloud(
        draw,
        390,
        135,
        0.75,
    );

    draw_cloud(
        draw,
        760,
        80,
        0.9,
    );
}

pub fn draw_scenery(
    draw: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
    map: &[Vec<char>],
    player: &Player,
    fov: f32,
    rays: &[RayHit],
) {
    if rays.is_empty() {
        return;
    }

    let mut objects =
        Vec::<Scenery>::new();

    for (row, line)
        in map.iter()
            .enumerate()
    {
        for (col, tile)
            in line.iter()
                .enumerate()
        {
            if matches!(
                *tile,
                'T' | 'F'
            ) {
                objects.push(
                    Scenery {
                        x:
                            col as f32
                            + 0.5,
                        y:
                            row as f32
                            + 0.5,
                        kind:
                            *tile,
                    }
                );
            }
        }
    }

    objects.sort_by(
        |a, b| {
            let da =
                (
                    a.x - player.x
                ).powi(2)
                + (
                    a.y - player.y
                ).powi(2);

            let db =
                (
                    b.x - player.x
                ).powi(2)
                + (
                    b.y - player.y
                ).powi(2);

            db.partial_cmp(&da)
                .unwrap_or(
                    std::cmp::Ordering::Equal
                )
        }
    );

    for object in objects {
        draw_object(
            draw,
            width,
            height,
            player,
            fov,
            rays,
            object,
        );
    }
}

pub fn draw_checkpoint(
    draw: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
    player: &Player,
    race: &Race,
    fov: f32,
    rays: &[RayHit],
) {
    let Some(checkpoint) =
        race.active_checkpoint()
    else {
        return;
    };

    if rays.is_empty() {
        return;
    }

    let dx =
        checkpoint.x
        - player.x;

    let dy =
        checkpoint.y
        - player.y;

    let raw_distance =
        (
            dx * dx
            + dy * dy
        ).sqrt();

    if raw_distance < 0.1 {
        return;
    }

    let object_angle =
        dy.atan2(dx);

    let relative_angle =
        normalize_angle(
            object_angle
            - player.angle
        );

    if relative_angle.abs()
        > fov * 0.62
    {
        return;
    }

    let corrected_distance =
        raw_distance
        * relative_angle.cos();

    if corrected_distance <= 0.05 {
        return;
    }

    let screen_x =
        width as f32 / 2.0
        + (
            relative_angle
            / (fov / 2.0)
        )
        * (
            width as f32 / 2.0
        );

    let ray_index =
        (
            screen_x
            / width as f32
            * rays.len() as f32
        ) as isize;

    if ray_index < 0
        || ray_index
            >= rays.len() as isize
    {
        return;
    }

    let wall_distance =
        rays[
            ray_index as usize
        ]
        .corrected_distance;

    // Si hay pared/seto antes, el checkpoint no atraviesa.
    if corrected_distance
        > wall_distance + 0.18
    {
        return;
    }

    let size =
        (
            600.0
            / corrected_distance
        )
        .clamp(
            35.0,
            260.0,
        );

    let horizon =
        height as f32 / 2.0;

    let ground_y =
        horizon
        + (
            95.0
            / corrected_distance
                .max(0.35)
        )
        .clamp(
            0.0,
            175.0,
        );

    let top_y =
        ground_y
        - size;

    let half_width =
        size * 0.32;

    let glow =
        Color::new(
            255,
            225,
            60,
            70,
        );

    let bright =
        Color::new(
            255,
            230,
            70,
            255,
        );

    // Halo grande
    draw.draw_circle(
        screen_x as i32,
        (
            top_y
            + size * 0.48
        ) as i32,
        size * 0.43,
        glow,
    );

    // Dos pilares
    draw.draw_rectangle(
        (
            screen_x
            - half_width
        ) as i32,
        top_y as i32,
        (size * 0.08) as i32,
        size as i32,
        bright,
    );

    draw.draw_rectangle(
        (
            screen_x
            + half_width
            - size * 0.08
        ) as i32,
        top_y as i32,
        (size * 0.08) as i32,
        size as i32,
        bright,
    );

    // Parte superior
    draw.draw_rectangle(
        (
            screen_x
            - half_width
        ) as i32,
        top_y as i32,
        (
            half_width
            * 2.0
        ) as i32,
        (size * 0.08) as i32,
        bright,
    );

    // Etiqueta
    let label =
        "CHECKPOINT";

    let font_size =
        (size * 0.10)
            .clamp(
                12.0,
                24.0,
            ) as i32;

    let text_width =
        draw.measure_text(
            label,
            font_size,
        );

    draw.draw_text(
        label,
        (
            screen_x
            - text_width as f32
                / 2.0
        ) as i32,
        (
            top_y
            - font_size as f32
            - 6.0
        ) as i32,
        font_size,
        Color::YELLOW,
    );
}

fn draw_object(
    draw: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
    player: &Player,
    fov: f32,
    rays: &[RayHit],
    object: Scenery,
) {
    let dx =
        object.x
        - player.x;

    let dy =
        object.y
        - player.y;

    let raw_distance =
        (
            dx * dx
            + dy * dy
        ).sqrt();

    if raw_distance < 0.2 {
        return;
    }

    let object_angle =
        dy.atan2(dx);

    let relative_angle =
        normalize_angle(
            object_angle
            - player.angle
        );

    if relative_angle.abs()
        > fov * 0.60
    {
        return;
    }

    let corrected_distance =
        raw_distance
        * relative_angle.cos();

    if corrected_distance <= 0.05 {
        return;
    }

    let screen_x =
        width as f32 / 2.0
        + (
            relative_angle
            / (fov / 2.0)
        )
        * (
            width as f32 / 2.0
        );

    let ray_index =
        (
            screen_x
            / width as f32
            * rays.len() as f32
        ) as isize;

    if ray_index < 0
        || ray_index
            >= rays.len() as isize
    {
        return;
    }

    let wall_distance =
        rays[
            ray_index as usize
        ]
        .corrected_distance;

    if corrected_distance
        > wall_distance + 0.18
    {
        return;
    }

    match object.kind {
        'T' => {
            let size =
                (
                    470.0
                    / corrected_distance
                )
                .clamp(
                    22.0,
                    300.0,
                );

            draw_tree(
                draw,
                height,
                screen_x,
                size,
                corrected_distance,
            );
        }

        'F' => {
            let size =
                (
                    210.0
                    / corrected_distance
                )
                .clamp(
                    8.0,
                    90.0,
                );

            draw_flowers(
                draw,
                height,
                screen_x,
                size,
                corrected_distance,
            );
        }

        _ => {}
    }
}

fn draw_tree(
    draw: &mut RaylibDrawHandle,
    height: i32,
    screen_x: f32,
    size: f32,
    distance: f32,
) {
    let horizon =
        height as f32 / 2.0;

    let ground_y =
        horizon
        + (
            95.0
            / distance.max(0.35)
        )
        .clamp(
            0.0,
            180.0,
        );

    let trunk_height =
        size * 0.48;

    let trunk_width =
        size * 0.18;

    let crown_radius =
        size * 0.34;

    let shade =
        (
            1.0
            / (
                1.0
                + distance
                    * 0.05
            )
        )
        .clamp(
            0.50,
            1.0,
        );

    draw.draw_ellipse(
        screen_x as i32,
        ground_y as i32,
        size * 0.30,
        size * 0.07,
        Color::new(
            20,
            45,
            20,
            95,
        ),
    );

    draw.draw_rectangle(
        (
            screen_x
            - trunk_width / 2.0
        ) as i32,
        (
            ground_y
            - trunk_height
        ) as i32,
        trunk_width as i32,
        trunk_height as i32,
        shade_color(
            Color::new(
                115,
                72,
                38,
                255,
            ),
            shade,
        ),
    );

    let crown_y =
        ground_y
        - trunk_height
        - crown_radius * 0.55;

    let leaves_dark =
        shade_color(
            Color::new(
                38,
                118,
                48,
                255,
            ),
            shade,
        );

    let leaves =
        shade_color(
            Color::new(
                58,
                155,
                65,
                255,
            ),
            shade,
        );

    draw.draw_circle(
        (
            screen_x
            - crown_radius
                * 0.55
        ) as i32,
        crown_y as i32,
        crown_radius,
        leaves_dark,
    );

    draw.draw_circle(
        (
            screen_x
            + crown_radius
                * 0.55
        ) as i32,
        crown_y as i32,
        crown_radius,
        leaves_dark,
    );

    draw.draw_circle(
        screen_x as i32,
        (
            crown_y
            - crown_radius
                * 0.55
        ) as i32,
        crown_radius
            * 1.10,
        leaves,
    );

    draw.draw_circle(
        screen_x as i32,
        crown_y as i32,
        crown_radius * 0.95,
        leaves,
    );
}

fn draw_flowers(
    draw: &mut RaylibDrawHandle,
    height: i32,
    screen_x: f32,
    size: f32,
    distance: f32,
) {
    let horizon =
        height as f32 / 2.0;

    let ground_y =
        horizon
        + (
            92.0
            / distance.max(0.4)
        )
        .clamp(
            0.0,
            170.0,
        );

    let colors = [
        Color::new(
            240,
            95,
            150,
            255,
        ),
        Color::new(
            245,
            215,
            70,
            255,
        ),
        Color::new(
            245,
            245,
            245,
            255,
        ),
    ];

    for i in 0..5 {
        let offset =
            (
                i as f32
                - 2.0
            )
            * size
            * 0.16;

        let flower_height =
            size
            * (
                0.45
                + (i % 2) as f32
                    * 0.12
            );

        let x =
            screen_x
            + offset;

        let top =
            ground_y
            - flower_height;

        draw.draw_line(
            x as i32,
            ground_y as i32,
            x as i32,
            top as i32,
            Color::new(
                45,
                125,
                50,
                255,
            ),
        );

        draw.draw_circle(
            x as i32,
            top as i32,
            (size * 0.09)
                .max(2.0),
            colors[
                i % colors.len()
            ],
        );
    }
}

fn draw_cloud(
    draw: &mut RaylibDrawHandle,
    x: i32,
    y: i32,
    scale: f32,
) {
    let color =
        Color::new(
            240,
            245,
            250,
            235,
        );

    draw.draw_circle(
        x,
        y,
        26.0 * scale,
        color,
    );

    draw.draw_circle(
        x
        + (30.0 * scale)
            as i32,
        y
        - (8.0 * scale)
            as i32,
        31.0 * scale,
        color,
    );

    draw.draw_circle(
        x
        + (60.0 * scale)
            as i32,
        y,
        24.0 * scale,
        color,
    );

    draw.draw_rectangle(
        x,
        y,
        (60.0 * scale)
            as i32,
        (24.0 * scale)
            as i32,
        color,
    );
}

fn normalize_angle(
    mut angle: f32,
) -> f32 {
    while angle
        > std::f32::consts::PI
    {
        angle -=
            std::f32::consts::TAU;
    }

    while angle
        < -std::f32::consts::PI
    {
        angle +=
            std::f32::consts::TAU;
    }

    angle
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