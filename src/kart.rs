use raylib::prelude::*;

pub fn draw_kart(
    draw: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
    velocity: f32,
    steering: f32,
) {
    let center_x =
        width / 2;

    let bottom =
        height - 20;

    let bounce =
        (
            velocity.abs()
            * 1.5
        )
        .min(3.0)
        as i32;

    let turn_offset =
        (
            steering
            * 24.0
        )
        as i32;

    let body_shift =
        (
            steering
            * 10.0
        )
        as i32;

    let center =
        center_x
        + turn_offset;

    let y =
        bottom
        - bounce;

    draw.draw_ellipse(
        center,
        y - 8,
        85.0,
        18.0,
        Color::new(
            20,
            20,
            20,
            150,
        ),
    );

    draw.draw_rectangle(
        center - 82 - body_shift,
        y - 69,
        28,
        55,
        Color::new(
            24,
            25,
            28,
            255,
        ),
    );

    draw.draw_rectangle(
        center + 54 - body_shift,
        y - 69,
        28,
        55,
        Color::new(
            24,
            25,
            28,
            255,
        ),
    );

    draw.draw_rectangle(
        center - 67,
        y - 42,
        134,
        24,
        Color::new(
            180,
            35,
            35,
            255,
        ),
    );

    draw.draw_rectangle(
        center - 55 + body_shift,
        y - 90,
        110,
        50,
        Color::new(
            220,
            43,
            42,
            255,
        ),
    );

    draw.draw_rectangle(
        center - 39 + body_shift,
        y - 111,
        78,
        27,
        Color::new(
            240,
            57,
            49,
            255,
        ),
    );

    draw.draw_rectangle(
        center - 27 + body_shift,
        y - 122,
        54,
        35,
        Color::new(
            35,
            38,
            44,
            255,
        ),
    );

    draw.draw_circle(
        center + body_shift,
        y - 136,
        24.0,
        Color::new(
            245,
            185,
            72,
            255,
        ),
    );

    draw.draw_rectangle(
        center - 22 + body_shift,
        y - 153,
        44,
        18,
        Color::RED,
    );

    draw.draw_rectangle(
        center - 14 + body_shift,
        y - 138,
        28,
        7,
        Color::SKYBLUE,
    );

    draw.draw_rectangle(
        center - 19,
        y - 43,
        38,
        16,
        Color::RAYWHITE,
    );

    draw.draw_text(
        "RUST",
        center - 16,
        y - 42,
        12,
        Color::BLACK,
    );
}