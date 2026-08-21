use raylib::prelude::*;

pub fn draw_kart(
    draw: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
    velocity: f32,
    steering: f32,
    drift: f32,
    boost_flash: f32,
    kart_color: Color,
    vehicle_index: usize,
    time: f32,
) {
    if vehicle_index % 2 == 1 {
        draw_motorcycle(
            draw,
            width,
            height,
            velocity,
            steering,
            drift,
            boost_flash,
            kart_color,
            time,
        );
        return;
    }

    let scale = (width as f32 / 1200.0)
        .min(height as f32 / 720.0)
        .clamp(1.08, 1.48);

    let center_x = width as f32 / 2.0;

    let bottom = height as f32 - 20.0 * scale;

    let bounce = (velocity.abs() * 1.8).min(5.0) * scale;

    let turn_offset = steering * 28.0 * scale;

    let body_shift = steering * 13.0 * scale;

    let center = center_x + turn_offset;

    let y = bottom - bounce;

    if boost_flash > 0.01 {
        let alpha = (150.0 * boost_flash) as u8;

        for i in 0..7 {
            let offset = (i as f32 - 3.0) * 34.0 * scale;

            draw.draw_line(
                (center + offset) as i32,
                (y - 180.0 * scale) as i32,
                (center + offset * 0.45) as i32,
                (y - 40.0 * scale) as i32,
                Color::new(255, 220, 60, alpha),
            );
        }

        for i in 0..4 {
            let flame_w = (18.0 + i as f32 * 6.0) * scale;

            let flame_h = (46.0 - i as f32 * 5.0) * scale * boost_flash;

            let flame_x = center + (i as f32 - 1.5) * 28.0 * scale;

            draw.draw_triangle(
                Vector2::new(flame_x, y - 25.0 * scale),
                Vector2::new(flame_x - flame_w / 2.0, y + flame_h),
                Vector2::new(flame_x + flame_w / 2.0, y + flame_h),
                Color::new(255, 138, 35, (190.0 * boost_flash) as u8),
            );
        }
    }

    if drift > 0.18 && velocity.abs() > 2.0 {
        let side = if steering >= 0.0 { -1.0 } else { 1.0 };

        for i in 0..7 {
            let t = i as f32;

            let wave = (time * 16.0 + t).sin() * 5.0 * scale;

            let smoke_x = center + side * (68.0 + t * 10.0) * scale + wave;

            let smoke_y = y - (18.0 + t * 8.0) * scale;

            let alpha = (90.0 * drift) as u8;

            draw.draw_circle(
                smoke_x as i32,
                smoke_y as i32,
                (9.0 + t * 1.7) * scale,
                Color::new(230, 230, 220, alpha),
            );
        }

        let mark_color = Color::new(20, 20, 20, (150.0 * drift) as u8);

        draw.draw_line(
            (center - 64.0 * scale) as i32,
            (y - 4.0 * scale) as i32,
            (center - 118.0 * scale - steering * 24.0 * scale) as i32,
            (y + 18.0 * scale) as i32,
            mark_color,
        );

        draw.draw_line(
            (center + 64.0 * scale) as i32,
            (y - 4.0 * scale) as i32,
            (center + 118.0 * scale - steering * 24.0 * scale) as i32,
            (y + 18.0 * scale) as i32,
            mark_color,
        );

        draw.draw_line(
            (center - 82.0 * scale) as i32,
            (y + 12.0 * scale) as i32,
            (center - 150.0 * scale - steering * 36.0 * scale) as i32,
            (y + 38.0 * scale) as i32,
            mark_color,
        );

        draw.draw_line(
            (center + 82.0 * scale) as i32,
            (y + 12.0 * scale) as i32,
            (center + 150.0 * scale - steering * 36.0 * scale) as i32,
            (y + 38.0 * scale) as i32,
            mark_color,
        );
    }

    draw.draw_ellipse(
        center as i32,
        (y - 8.0 * scale) as i32,
        85.0 * scale,
        18.0 * scale,
        Color::new(20, 20, 20, 150),
    );

    draw.draw_rectangle(
        (center - 82.0 * scale - body_shift) as i32,
        (y - 69.0 * scale) as i32,
        (28.0 * scale) as i32,
        (55.0 * scale) as i32,
        Color::new(24, 25, 28, 255),
    );

    draw.draw_rectangle(
        (center + 54.0 * scale - body_shift) as i32,
        (y - 69.0 * scale) as i32,
        (28.0 * scale) as i32,
        (55.0 * scale) as i32,
        Color::new(24, 25, 28, 255),
    );

    draw.draw_rectangle(
        (center - 67.0 * scale) as i32,
        (y - 42.0 * scale) as i32,
        (134.0 * scale) as i32,
        (24.0 * scale) as i32,
        shade_color(kart_color, 0.74),
    );

    draw.draw_rectangle(
        (center - 55.0 * scale + body_shift) as i32,
        (y - 90.0 * scale) as i32,
        (110.0 * scale) as i32,
        (50.0 * scale) as i32,
        kart_color,
    );

    draw.draw_rectangle(
        (center - 39.0 * scale + body_shift) as i32,
        (y - 111.0 * scale) as i32,
        (78.0 * scale) as i32,
        (27.0 * scale) as i32,
        shade_color(kart_color, 1.14),
    );

    draw.draw_rectangle(
        (center - 27.0 * scale + body_shift) as i32,
        (y - 122.0 * scale) as i32,
        (54.0 * scale) as i32,
        (35.0 * scale) as i32,
        Color::new(35, 38, 44, 255),
    );

    draw.draw_circle(
        (center + body_shift) as i32,
        (y - 136.0 * scale) as i32,
        24.0 * scale,
        Color::new(245, 185, 72, 255),
    );

    draw.draw_rectangle(
        (center - 22.0 * scale + body_shift) as i32,
        (y - 153.0 * scale) as i32,
        (44.0 * scale) as i32,
        (18.0 * scale) as i32,
        shade_color(kart_color, 0.92),
    );

    draw.draw_rectangle(
        (center - 14.0 * scale + body_shift) as i32,
        (y - 138.0 * scale) as i32,
        (28.0 * scale) as i32,
        (7.0 * scale) as i32,
        Color::SKYBLUE,
    );

    draw.draw_rectangle(
        (center - 19.0 * scale) as i32,
        (y - 43.0 * scale) as i32,
        (38.0 * scale) as i32,
        (16.0 * scale) as i32,
        Color::RAYWHITE,
    );

    draw.draw_text(
        "RUST",
        (center - 16.0 * scale) as i32,
        (y - 42.0 * scale) as i32,
        (12.0 * scale) as i32,
        Color::BLACK,
    );
}

fn draw_motorcycle(
    draw: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
    velocity: f32,
    steering: f32,
    drift: f32,
    boost_flash: f32,
    bike_color: Color,
    time: f32,
) {
    let scale = (width as f32 / 1200.0)
        .min(height as f32 / 720.0)
        .clamp(1.08, 1.48);

    let center_x = width as f32 / 2.0;
    let bottom = height as f32 - 20.0 * scale;
    let bounce = (velocity.abs() * 1.9).min(5.5) * scale;
    let turn_offset = steering * 34.0 * scale;
    let lean = steering * 28.0 * scale;
    let center = center_x + turn_offset;
    let y = bottom - bounce;

    if boost_flash > 0.01 {
        for i in 0..4 {
            let flame_h = (42.0 - i as f32 * 5.0) * scale * boost_flash;
            let flame_x = center + (i as f32 - 1.5) * 18.0 * scale;

            draw.draw_triangle(
                Vector2::new(flame_x, y - 16.0 * scale),
                Vector2::new(flame_x - 9.0 * scale, y + flame_h),
                Vector2::new(flame_x + 9.0 * scale, y + flame_h),
                Color::new(255, 142, 38, (190.0 * boost_flash) as u8),
            );
        }
    }

    if drift > 0.18 && velocity.abs() > 2.0 {
        let side = if steering >= 0.0 { -1.0 } else { 1.0 };

        for i in 0..5 {
            let t = i as f32;
            let wave = (time * 15.0 + t).sin() * 4.0 * scale;
            let smoke_x = center + side * (52.0 + t * 10.0) * scale + wave;
            let smoke_y = y - (13.0 + t * 8.0) * scale;

            draw.draw_circle(
                smoke_x as i32,
                smoke_y as i32,
                (8.0 + t * 1.5) * scale,
                Color::new(230, 230, 220, (85.0 * drift) as u8),
            );
        }
    }

    draw.draw_ellipse(
        center as i32,
        (y - 6.0 * scale) as i32,
        72.0 * scale,
        15.0 * scale,
        Color::new(20, 20, 20, 145),
    );

    draw.draw_circle(
        (center - 58.0 * scale) as i32,
        (y - 28.0 * scale) as i32,
        23.0 * scale,
        Color::new(22, 23, 26, 255),
    );

    draw.draw_circle(
        (center + 58.0 * scale) as i32,
        (y - 28.0 * scale) as i32,
        23.0 * scale,
        Color::new(22, 23, 26, 255),
    );

    draw.draw_line_ex(
        Vector2::new(center - 58.0 * scale, y - 28.0 * scale),
        Vector2::new(center - 8.0 * scale + lean * 0.35, y - 78.0 * scale),
        11.0 * scale,
        shade_color(bike_color, 0.78),
    );

    draw.draw_line_ex(
        Vector2::new(center + 58.0 * scale, y - 28.0 * scale),
        Vector2::new(center - 8.0 * scale + lean * 0.35, y - 78.0 * scale),
        11.0 * scale,
        bike_color,
    );

    draw.draw_rectangle(
        (center - 42.0 * scale + lean * 0.25) as i32,
        (y - 96.0 * scale) as i32,
        (84.0 * scale) as i32,
        (26.0 * scale) as i32,
        shade_color(bike_color, 1.12),
    );

    draw.draw_rectangle(
        (center - 21.0 * scale + lean) as i32,
        (y - 137.0 * scale) as i32,
        (42.0 * scale) as i32,
        (56.0 * scale) as i32,
        Color::new(32, 36, 42, 255),
    );

    draw.draw_circle(
        (center + lean) as i32,
        (y - 163.0 * scale) as i32,
        24.0 * scale,
        Color::new(245, 185, 72, 255),
    );

    draw.draw_rectangle(
        (center - 22.0 * scale + lean) as i32,
        (y - 181.0 * scale) as i32,
        (44.0 * scale) as i32,
        (19.0 * scale) as i32,
        shade_color(bike_color, 0.92),
    );

    draw.draw_rectangle(
        (center - 15.0 * scale + lean) as i32,
        (y - 166.0 * scale) as i32,
        (30.0 * scale) as i32,
        (8.0 * scale) as i32,
        Color::SKYBLUE,
    );
}

fn shade_color(color: Color, factor: f32) -> Color {
    Color::new(
        (color.r as f32 * factor).clamp(0.0, 255.0) as u8,
        (color.g as f32 * factor).clamp(0.0, 255.0) as u8,
        (color.b as f32 * factor).clamp(0.0, 255.0) as u8,
        color.a,
    )
}
