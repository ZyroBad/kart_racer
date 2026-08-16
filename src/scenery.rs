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
                'T' | 'F' | 'O' | 'C' | 'B' | 'A' | 'G'
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

        'O' => {
            let size =
                (180.0 / corrected_distance)
                .clamp(12.0, 95.0);

            draw_cone(
                draw,
                height,
                screen_x,
                size,
                corrected_distance,
            );
        }

        'C' => {
            let size =
                (260.0 / corrected_distance)
                .clamp(18.0, 140.0);

            draw_crate(
                draw,
                height,
                screen_x,
                size,
                corrected_distance,
            );
        }

        'B' => {
            let size =
                (360.0 / corrected_distance)
                .clamp(26.0, 190.0);

            draw_barrier(
                draw,
                height,
                screen_x,
                size,
                corrected_distance,
            );
        }

        'A' => {
            let size =
                (390.0 / corrected_distance)
                .clamp(24.0, 210.0);

            draw_statue(
                draw,
                height,
                screen_x,
                size,
                corrected_distance,
            );
        }

        'G' => {
            let size =
                (520.0 / corrected_distance)
                .clamp(34.0, 260.0);

            draw_grandstand(
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


fn object_ground_y(
    height: i32,
    distance: f32,
) -> f32 {
    height as f32 / 2.0
        + (95.0 / distance.max(0.35))
            .clamp(0.0, 175.0)
}

fn draw_cone(
    draw: &mut RaylibDrawHandle,
    height: i32,
    x: f32,
    size: f32,
    distance: f32,
) {
    let ground_y =
        object_ground_y(height, distance);

    let orange =
        Color::new(245, 125, 25, 255);

    let white =
        Color::new(245, 245, 235, 255);

    draw.draw_triangle(
        Vector2::new(x, ground_y - size),
        Vector2::new(x - size * 0.35, ground_y),
        Vector2::new(x + size * 0.35, ground_y),
        orange,
    );

    draw.draw_rectangle(
        (x - size * 0.20) as i32,
        (ground_y - size * 0.45) as i32,
        (size * 0.40) as i32,
        (size * 0.12) as i32,
        white,
    );

    draw.draw_rectangle(
        (x - size * 0.43) as i32,
        ground_y as i32,
        (size * 0.86) as i32,
        (size * 0.10) as i32,
        Color::new(40, 40, 45, 255),
    );
}

fn draw_crate(
    draw: &mut RaylibDrawHandle,
    height: i32,
    x: f32,
    size: f32,
    distance: f32,
) {
    let ground_y =
        object_ground_y(height, distance);

    let left =
        x - size * 0.5;

    let top =
        ground_y - size;

    let wood =
        Color::new(155, 102, 55, 255);

    let dark =
        Color::new(95, 58, 32, 255);

    draw.draw_rectangle(
        left as i32,
        top as i32,
        size as i32,
        size as i32,
        wood,
    );

    draw.draw_rectangle_lines(
        left as i32,
        top as i32,
        size as i32,
        size as i32,
        dark,
    );

    draw.draw_line(
        left as i32,
        top as i32,
        (left + size) as i32,
        ground_y as i32,
        dark,
    );

    draw.draw_line(
        (left + size) as i32,
        top as i32,
        left as i32,
        ground_y as i32,
        dark,
    );
}

fn draw_barrier(
    draw: &mut RaylibDrawHandle,
    height: i32,
    x: f32,
    size: f32,
    distance: f32,
) {
    let ground_y =
        object_ground_y(height, distance);

    let width =
        size * 1.25;

    let bar_h =
        size * 0.30;

    let left =
        x - width / 2.0;

    let top =
        ground_y - size * 0.65;

    draw.draw_rectangle(
        left as i32,
        top as i32,
        width as i32,
        bar_h as i32,
        Color::RAYWHITE,
    );

    let stripe_w =
        width / 5.0;

    for i in 0..5 {
        if i % 2 == 0 {
            draw.draw_rectangle(
                (left + i as f32 * stripe_w) as i32,
                top as i32,
                stripe_w as i32,
                bar_h as i32,
                Color::RED,
            );
        }
    }

    let leg_w =
        size * 0.10;

    draw.draw_rectangle(
        (left + size * 0.15) as i32,
        (top + bar_h) as i32,
        leg_w as i32,
        (size * 0.35) as i32,
        Color::new(55, 55, 60, 255),
    );

    draw.draw_rectangle(
        (left + width - size * 0.25) as i32,
        (top + bar_h) as i32,
        leg_w as i32,
        (size * 0.35) as i32,
        Color::new(55, 55, 60, 255),
    );
}

fn draw_statue(
    draw: &mut RaylibDrawHandle,
    height: i32,
    x: f32,
    size: f32,
    distance: f32,
) {
    let ground_y =
        object_ground_y(height, distance);

    let stone =
        Color::new(185, 185, 195, 255);

    let dark =
        Color::new(120, 120, 130, 255);

    draw.draw_rectangle(
        (x - size * 0.32) as i32,
        (ground_y - size * 0.22) as i32,
        (size * 0.64) as i32,
        (size * 0.22) as i32,
        dark,
    );

    draw.draw_rectangle(
        (x - size * 0.20) as i32,
        (ground_y - size * 0.70) as i32,
        (size * 0.40) as i32,
        (size * 0.50) as i32,
        stone,
    );

    draw.draw_circle(
        x as i32,
        (ground_y - size * 0.82) as i32,
        size * 0.20,
        stone,
    );
}

fn draw_grandstand(
    draw: &mut RaylibDrawHandle,
    height: i32,
    x: f32,
    size: f32,
    distance: f32,
) {
    let ground_y =
        object_ground_y(height, distance);

    let width =
        size * 1.35;

    let left =
        x - width / 2.0;

    let top =
        ground_y - size * 0.72;

    let shade =
        (
            1.0
            / (
                1.0
                + distance * 0.04
            )
        )
        .clamp(
            0.55,
            1.0,
        );

    draw.draw_rectangle(
        left as i32,
        top as i32,
        width as i32,
        (size * 0.55) as i32,
        shade_color(
            Color::new(
                42,
                52,
                72,
                255,
            ),
            shade,
        ),
    );

    for row in 0..3 {
        let y =
            top
            + row as f32
                * size
                * 0.16;

        draw.draw_rectangle(
            left as i32,
            y as i32,
            width as i32,
            (size * 0.06) as i32,
            shade_color(
                Color::new(
                    220,
                    55,
                    55,
                    255,
                ),
                shade,
            ),
        );
    }

    let colors = [
        Color::YELLOW,
        Color::SKYBLUE,
        Color::RAYWHITE,
        Color::GREEN,
    ];

    for i in 0..8 {
        let seat_x =
            left
            + width * 0.12
            + i as f32
                * width
                * 0.10;

        let seat_y =
            top
            + size * 0.22
            + (i % 3) as f32
                * size
                * 0.10;

        draw.draw_circle(
            seat_x as i32,
            seat_y as i32,
            (size * 0.035)
                .max(2.0),
            colors[
                i % colors.len()
            ],
        );
    }

    draw.draw_rectangle(
        left as i32,
        (ground_y - size * 0.18) as i32,
        width as i32,
        (size * 0.18) as i32,
        shade_color(
            Color::new(
                245,
                245,
                245,
                255,
            ),
            shade,
        ),
    );
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
