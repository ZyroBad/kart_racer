use raylib::prelude::*;

use crate::{kart, minimap, player::Player, race::Race, raycaster, scenery};

pub struct Framebuffer;

impl Framebuffer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        map: &[Vec<char>],
        player: &Player,
        race: &Race,
        fov: f32,
        number_of_rays: usize,
        show_start_screen: bool,
        show_track_select_screen: bool,
        kart_color: Color,
        kart_color_name: &str,
        track_name: &str,
        selected_track: usize,
        track_count: usize,
        track_select_option: usize,
        vehicle_index: usize,
        vehicle_name: &str,
        music_enabled: bool,
        sfx_enabled: bool,
        countdown_timer: Option<f32>,
        show_pause_screen: bool,
        pause_menu_option: usize,
        start_menu_option: usize,
        show_controls: bool,
    ) {
        // IMPORTANTE:
        // Limpiamos TODA la ventana cada frame.
        //
        // Esto evita que al maximizar queden partes negras
        // o imágenes viejas del kart en pantalla.
        draw.clear_background(Color::BLACK);

        if show_start_screen {
            self.draw_start_screen(
                draw,
                width,
                height,
                kart_color,
                kart_color_name,
                music_enabled,
                sfx_enabled,
                start_menu_option,
                show_controls,
            );

            return;
        }

        if show_track_select_screen {
            self.draw_track_select_screen(
                draw,
                width,
                height,
                selected_track,
                track_count,
                track_select_option,
                track_name,
                kart_color,
                vehicle_index,
                vehicle_name,
            );

            return;
        }

        let is_city_track = selected_track == 1;

        scenery::draw_sky(draw, width, height, is_city_track);

        raycaster::draw_floor(draw, width, height, map, player, fov, is_city_track);

        let rays = raycaster::cast_all_rays(map, player, fov, number_of_rays);

        raycaster::draw_walls(draw, width, height, &rays, fov, is_city_track);

        scenery::draw_scenery(draw, width, height, map, player, fov, &rays);

        scenery::draw_checkpoint(draw, width, height, player, race, fov, &rays);

        minimap::draw_minimap(draw, width, map, player, race, &rays);

        self.draw_checkpoint_guide(draw, width, player, race);

        kart::draw_kart(
            draw,
            width,
            height,
            player.velocity,
            player.steering,
            player.drift,
            player.boost_flash,
            kart_color,
            vehicle_index,
            race.race_time(),
        );

        self.draw_boost_overlay(draw, width, height, player);

        self.draw_race_hud(draw, width, height, player, race);

        self.draw_race_event(draw, width, height, race);

        if let Some(timer) = countdown_timer {
            self.draw_countdown(draw, width, height, timer);
        }

        if show_pause_screen {
            self.draw_pause_screen(
                draw,
                width,
                height,
                pause_menu_option,
                music_enabled,
                sfx_enabled,
            );
        }
    }

    fn draw_race_hud(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        player: &Player,
        race: &Race,
    ) {
        draw.draw_rectangle(18, 18, 300, 200, Color::new(15, 18, 22, 190));

        draw.draw_text(
            &format!("VUELTA {}/{}", race.current_lap(), race.total_laps(),),
            30,
            28,
            26,
            Color::RAYWHITE,
        );

        if !race.finished() {
            let objective = if race.active_checkpoint_label() == "META" {
                "META".to_string()
            } else {
                format!(
                    "CHECKPOINT {}/{}",
                    race.current_checkpoint() + 1,
                    race.checkpoint_count() - 1,
                )
            };

            draw.draw_text(&objective, 30, 62, 18, Color::YELLOW);
        }

        draw.draw_text(
            &format!("VUELTA: {}", format_time(race.lap_time()),),
            30,
            92,
            18,
            Color::RAYWHITE,
        );

        draw.draw_text(
            &format!("TOTAL: {}", format_time(race.race_time()),),
            30,
            120,
            18,
            Color::LIGHTGRAY,
        );

        let best_text = match race.best_lap_time() {
            Some(time) => format!("MEJOR: {}", format_time(time)),

            None => "MEJOR: --:--.---".to_string(),
        };

        draw.draw_text(&best_text, 30, 148, 18, Color::GREEN);

        if let Some(time) = race.last_lap_time() {
            draw.draw_text(
                &format!("ULTIMA: {}", format_time(time)),
                30,
                176,
                16,
                Color::ORANGE,
            );
        }

        draw.draw_fps(width - 100, height - 32);

        self.draw_speedometer(draw, width, height, player);

        if player.boost_flash > 0.1 {
            let boost = "BOOST";

            let boost_size = 24;

            let boost_width = draw.measure_text(boost, boost_size);

            draw.draw_text(
                boost,
                width - boost_width - 34,
                height - 74,
                boost_size,
                Color::YELLOW,
            );
        }

        if race.finished() {
            self.draw_finish_screen(draw, width, height, race);
        }
    }

    fn draw_finish_screen(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        race: &Race,
    ) {
        draw.draw_rectangle(0, 0, width, height, Color::new(0, 0, 0, 175));

        let title = "CARRERA COMPLETADA";

        let title_size = 44;

        let title_width = draw.measure_text(title, title_size);

        draw.draw_text(
            title,
            (width - title_width) / 2,
            height / 2 - 105,
            title_size,
            Color::YELLOW,
        );

        let total = format!("TIEMPO TOTAL: {}", format_time(race.race_time()));

        let total_size = 28;

        let total_width = draw.measure_text(&total, total_size);

        draw.draw_text(
            &total,
            (width - total_width) / 2,
            height / 2 - 25,
            total_size,
            Color::RAYWHITE,
        );

        let best = match race.best_lap_time() {
            Some(time) => format!("MEJOR VUELTA: {}", format_time(time)),

            None => "MEJOR VUELTA: --:--.---".to_string(),
        };

        let best_size = 24;

        let best_width = draw.measure_text(&best, best_size);

        draw.draw_text(
            &best,
            (width - best_width) / 2,
            height / 2 + 25,
            best_size,
            Color::GREEN,
        );

        let restart = "ENTER / R PARA JUGAR DE NUEVO   |   BACKSPACE AL MENU";

        let restart_size = 20;

        let restart_width = draw.measure_text(restart, restart_size);

        draw.draw_text(
            restart,
            (width - restart_width) / 2,
            height / 2 + 78,
            restart_size,
            Color::RAYWHITE,
        );
    }

    fn draw_speedometer(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        player: &Player,
    ) {
        let panel_width = 230;

        let panel_height = 74;

        let x = width - panel_width - 18;

        let y = height - panel_height - 18;

        draw.draw_rectangle(x, y, panel_width, panel_height, Color::new(12, 15, 20, 185));

        let speed = (player.velocity.abs() * 18.0) as i32;

        draw.draw_text(
            &format!("{} KM/H", speed),
            x + 18,
            y + 12,
            26,
            Color::RAYWHITE,
        );

        let bar_width = 190;

        let fill = (player.velocity.abs() / 9.0).clamp(0.0, 1.0);

        draw.draw_rectangle(x + 18, y + 50, bar_width, 10, Color::new(35, 42, 50, 255));

        draw.draw_rectangle(
            x + 18,
            y + 50,
            (bar_width as f32 * fill) as i32,
            10,
            if player.boost_flash > 0.1 {
                Color::YELLOW
            } else {
                Color::GREEN
            },
        );
    }

    fn draw_boost_overlay(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        player: &Player,
    ) {
        if player.boost_flash <= 0.08 {
            return;
        }

        let alpha = (135.0 * player.boost_flash) as u8;

        let mid_y = height as f32 * 0.58;

        for i in 0..7 {
            let y = mid_y + i as f32 * 34.0;

            let length = (110.0 + i as f32 * 15.0) * player.boost_flash;

            draw.draw_line_ex(
                Vector2::new(22.0, y),
                Vector2::new(22.0 + length, y - 18.0),
                3.0,
                Color::new(255, 230, 80, alpha),
            );

            draw.draw_line_ex(
                Vector2::new(width as f32 - 22.0, y),
                Vector2::new(width as f32 - 22.0 - length, y - 18.0),
                3.0,
                Color::new(255, 230, 80, alpha),
            );
        }
    }

    fn draw_race_event(&self, draw: &mut RaylibDrawHandle, width: i32, height: i32, race: &Race) {
        let Some(text) = race.event_text() else {
            return;
        };

        let timer = race.event_timer();

        if timer <= 0.0 {
            return;
        }

        let size = (34.0 + timer.min(0.4) * 18.0) as i32;

        let text_width = draw.measure_text(text, size);

        let x = (width - text_width) / 2;

        let y = height / 2 - 150;

        draw.draw_rectangle(
            x - 24,
            y - 12,
            text_width + 48,
            size + 24,
            Color::new(8, 10, 14, 170),
        );

        draw.draw_text(text, x, y, size, Color::YELLOW);
    }

    fn draw_countdown(&self, draw: &mut RaylibDrawHandle, width: i32, height: i32, timer: f32) {
        draw.draw_rectangle(0, 0, width, height, Color::new(0, 0, 0, 85));

        let text = if timer > 2.25 {
            "3"
        } else if timer > 1.25 {
            "2"
        } else if timer > 0.25 {
            "1"
        } else {
            "GO!"
        };

        let size = if text == "GO!" { 76 } else { 96 };

        let text_width = draw.measure_text(text, size);

        draw.draw_text(
            text,
            (width - text_width) / 2,
            height / 2 - 70,
            size,
            Color::YELLOW,
        );
    }

    fn draw_checkpoint_guide(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        player: &Player,
        race: &Race,
    ) {
        let Some(checkpoint) = race.active_checkpoint() else {
            return;
        };

        let dx = checkpoint.x - player.x;

        let dy = checkpoint.y - player.y;

        let distance = (dx * dx + dy * dy).sqrt();

        let target_angle = dy.atan2(dx);

        let relative_angle = normalize_angle(target_angle - player.angle);

        let center_x = width / 2;

        let top = 28;

        let (direction, instruction) = if relative_angle.abs() < 0.20 {
            ("^", "RECTO")
        } else if relative_angle > 0.0 {
            (">", "DERECHA")
        } else {
            ("<", "IZQUIERDA")
        };

        draw.draw_rectangle(center_x - 145, top, 290, 72, Color::new(10, 12, 18, 205));
        draw.draw_rectangle_lines(center_x - 145, top, 290, 72, Color::new(255, 225, 60, 220));

        let accent = race.active_checkpoint_color();

        draw.draw_text(direction, center_x - 12, top + 2, 34, accent);

        let text = format!(
            "{} {}: {:.0}m",
            instruction,
            race.active_checkpoint_label(),
            distance * 3.0
        );

        let text_size = 16;

        let text_width = draw.measure_text(&text, text_size);

        draw.draw_text(
            &text,
            center_x - text_width / 2,
            top + 44,
            text_size,
            Color::RAYWHITE,
        );
    }

    fn draw_start_screen(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        kart_color: Color,
        kart_color_name: &str,
        music_enabled: bool,
        sfx_enabled: bool,
        selected_option: usize,
        show_controls: bool,
    ) {
        self.draw_menu_scene(draw, width, height);
        self.draw_menu_title(draw, width, height);
        self.draw_menu_panel(
            draw,
            width,
            height,
            kart_color,
            kart_color_name,
            music_enabled,
            sfx_enabled,
            selected_option,
            show_controls,
        );

        self.draw_menu_kart_showcase(draw, width, height, kart_color);
    }

    fn draw_menu_scene(&self, draw: &mut RaylibDrawHandle, width: i32, height: i32) {
        draw.clear_background(Color::new(55, 166, 232, 255));

        let horizon = (height as f32 * 0.55) as i32;

        draw.draw_circle(width - 115, 86, 38.0, Color::new(250, 220, 75, 255));

        self.draw_menu_cloud(draw, width / 3, 86, 1.0);
        self.draw_menu_cloud(draw, width * 2 / 3, 136, 0.72);
        self.draw_menu_cloud(draw, width - 88, 146, 0.60);

        draw.draw_rectangle(0, horizon - 66, width, 66, Color::new(22, 122, 48, 255));

        draw.draw_rectangle(0, horizon - 18, width, 26, Color::new(14, 86, 34, 255));

        draw.draw_rectangle(
            0,
            horizon,
            width,
            height - horizon,
            Color::new(55, 150, 66, 255),
        );

        let road_top_y = horizon;

        draw.draw_triangle(
            Vector2::new((width / 2 + 18) as f32, road_top_y as f32),
            Vector2::new((width / 2 + 188) as f32, road_top_y as f32),
            Vector2::new((width / 2 + 330) as f32, height as f32),
            Color::new(187, 161, 115, 255),
        );

        draw.draw_triangle(
            Vector2::new((width / 2 + 18) as f32, road_top_y as f32),
            Vector2::new((width / 2 - 70) as f32, height as f32),
            Vector2::new((width / 2 + 330) as f32, height as f32),
            Color::new(194, 171, 128, 255),
        );

        self.draw_menu_finish_banner(draw, width, horizon);
        self.draw_menu_fountain(draw, width, horizon);
        self.draw_menu_tree(draw, width - 155, horizon + 8, 1.18);
        self.draw_menu_tree(draw, width - 320, horizon - 4, 0.82);
        self.draw_menu_flowers(draw, width - 385, horizon + 98);
        self.draw_menu_cone(draw, width / 2 + 292, horizon + 76, 1.0);
        self.draw_menu_cone(draw, width / 2 + 372, horizon + 45, 0.74);
        self.draw_menu_crate(draw, width / 2 + 210, horizon + 90, 1.0);
    }

    fn draw_menu_cloud(&self, draw: &mut RaylibDrawHandle, x: i32, y: i32, scale: f32) {
        let color = Color::new(238, 245, 250, 230);

        draw.draw_circle(x, y, 24.0 * scale, color);
        draw.draw_circle(
            x + (26.0 * scale) as i32,
            y + (4.0 * scale) as i32,
            20.0 * scale,
            color,
        );
        draw.draw_circle(
            x - (25.0 * scale) as i32,
            y + (7.0 * scale) as i32,
            18.0 * scale,
            color,
        );
        draw.draw_rectangle(
            x - (42.0 * scale) as i32,
            y + (8.0 * scale) as i32,
            (88.0 * scale) as i32,
            (16.0 * scale) as i32,
            color,
        );
    }

    fn draw_menu_finish_banner(&self, draw: &mut RaylibDrawHandle, width: i32, horizon: i32) {
        let x = width / 2 + 30;
        let y = horizon - 34;
        let w = 210;
        let h = 38;

        draw.draw_rectangle(x, y, w, h, Color::RAYWHITE);

        for i in 0..5 {
            if i % 2 == 0 {
                draw.draw_rectangle(x + i * w / 5, y, w / 5, h, Color::RED);
            }
        }

        draw.draw_rectangle(x - 8, y + h, 10, 72, Color::new(35, 43, 45, 255));
        draw.draw_rectangle(x + w - 2, y + h, 10, 72, Color::new(35, 43, 45, 255));
    }

    fn draw_menu_fountain(&self, draw: &mut RaylibDrawHandle, width: i32, horizon: i32) {
        let x = width - 225;
        let y = horizon + 108;

        draw.draw_ellipse(x, y + 24, 96.0, 26.0, Color::new(220, 230, 230, 255));
        draw.draw_ellipse(x, y + 20, 74.0, 18.0, Color::new(75, 190, 230, 255));
        draw.draw_rectangle(x - 18, y - 46, 36, 62, Color::new(168, 178, 186, 255));
        draw.draw_rectangle(x - 30, y - 18, 60, 18, Color::new(190, 196, 202, 255));

        for offset in [-28, 0, 28] {
            draw.draw_line(x, y - 48, x + offset, y + 10, Color::new(90, 210, 240, 210));
        }
    }

    fn draw_menu_tree(&self, draw: &mut RaylibDrawHandle, x: i32, ground_y: i32, scale: f32) {
        draw.draw_rectangle(
            x - (10.0 * scale) as i32,
            ground_y - (64.0 * scale) as i32,
            (20.0 * scale) as i32,
            (64.0 * scale) as i32,
            Color::new(115, 72, 38, 255),
        );

        let radius = 34.0 * scale;
        draw.draw_circle(
            x - (24.0 * scale) as i32,
            ground_y - (76.0 * scale) as i32,
            radius,
            Color::new(36, 118, 48, 255),
        );
        draw.draw_circle(
            x + (24.0 * scale) as i32,
            ground_y - (76.0 * scale) as i32,
            radius,
            Color::new(28, 100, 42, 255),
        );
        draw.draw_circle(
            x,
            ground_y - (106.0 * scale) as i32,
            radius * 1.15,
            Color::new(52, 150, 62, 255),
        );
    }

    fn draw_menu_flowers(&self, draw: &mut RaylibDrawHandle, x: i32, y: i32) {
        let colors = [
            Color::new(245, 215, 60, 255),
            Color::new(245, 95, 150, 255),
            Color::new(245, 245, 245, 255),
            Color::new(110, 95, 225, 255),
        ];

        for i in 0..18 {
            let fx = x + i * 18;
            let fy = y + (i % 4) * 8;
            draw.draw_line(fx, fy + 15, fx, fy, Color::new(35, 120, 50, 255));
            draw.draw_circle(fx, fy, 5.0, colors[i as usize % colors.len()]);
        }
    }

    fn draw_menu_cone(&self, draw: &mut RaylibDrawHandle, x: i32, y: i32, scale: f32) {
        let w = (22.0 * scale) as i32;
        let h = (54.0 * scale) as i32;

        draw.draw_triangle(
            Vector2::new(x as f32, (y - h) as f32),
            Vector2::new((x - w) as f32, y as f32),
            Vector2::new((x + w) as f32, y as f32),
            Color::new(245, 125, 25, 255),
        );
        draw.draw_rectangle(x - w / 2, y - h / 2, w, h / 7, Color::RAYWHITE);
        draw.draw_rectangle(x - w - 4, y, w * 2 + 8, 7, Color::new(38, 40, 44, 255));
    }

    fn draw_menu_crate(&self, draw: &mut RaylibDrawHandle, x: i32, y: i32, scale: f32) {
        let size = (52.0 * scale) as i32;
        let left = x - size / 2;
        let top = y - size;
        let wood = Color::new(155, 102, 55, 255);
        let dark = Color::new(95, 58, 32, 255);

        draw.draw_rectangle(left, top, size, size, wood);
        draw.draw_rectangle_lines(left, top, size, size, dark);
        draw.draw_line(left, top, left + size, top + size, dark);
        draw.draw_line(left + size, top, left, top + size, dark);
    }

    fn draw_menu_kart_showcase(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        kart_color: Color,
    ) {
        let scale = (width as f32 / 1200.0)
            .min(height as f32 / 720.0)
            .clamp(0.92, 1.16);
        let center = width / 2 + (260.0 * scale) as i32;
        let base_y = (height as f32 * 0.90) as i32;
        let sx = |value: f32| -> i32 { (value * scale) as i32 };

        draw.draw_ellipse(
            center,
            base_y + sx(18.0),
            142.0 * scale,
            25.0 * scale,
            Color::new(20, 20, 20, 125),
        );

        draw.draw_rectangle(
            center - sx(132.0),
            base_y - sx(72.0),
            sx(38.0),
            sx(88.0),
            Color::new(22, 23, 26, 255),
        );
        draw.draw_rectangle(
            center + sx(94.0),
            base_y - sx(72.0),
            sx(38.0),
            sx(88.0),
            Color::new(22, 23, 26, 255),
        );

        draw.draw_rectangle(
            center - sx(112.0),
            base_y - sx(30.0),
            sx(224.0),
            sx(34.0),
            Color::new(
                (kart_color.r as f32 * 0.72) as u8,
                (kart_color.g as f32 * 0.72) as u8,
                (kart_color.b as f32 * 0.72) as u8,
                255,
            ),
        );

        draw.draw_rectangle(
            center - sx(88.0),
            base_y - sx(108.0),
            sx(176.0),
            sx(82.0),
            kart_color,
        );
        draw.draw_rectangle(
            center - sx(62.0),
            base_y - sx(140.0),
            sx(124.0),
            sx(42.0),
            Color::new(
                (kart_color.r as f32 * 1.10).clamp(0.0, 255.0) as u8,
                (kart_color.g as f32 * 1.10).clamp(0.0, 255.0) as u8,
                (kart_color.b as f32 * 1.10).clamp(0.0, 255.0) as u8,
                255,
            ),
        );
        draw.draw_rectangle(
            center - sx(44.0),
            base_y - sx(158.0),
            sx(88.0),
            sx(58.0),
            Color::new(31, 36, 42, 255),
        );

        draw.draw_circle(
            center,
            base_y - sx(184.0),
            36.0 * scale,
            Color::new(245, 185, 72, 255),
        );
        draw.draw_rectangle(
            center - sx(34.0),
            base_y - sx(212.0),
            sx(68.0),
            sx(26.0),
            Color::new(
                (kart_color.r as f32 * 0.92) as u8,
                (kart_color.g as f32 * 0.92) as u8,
                (kart_color.b as f32 * 0.92) as u8,
                255,
            ),
        );
        draw.draw_rectangle(
            center - sx(25.0),
            base_y - sx(188.0),
            sx(50.0),
            sx(11.0),
            Color::SKYBLUE,
        );

        draw.draw_rectangle(
            center - sx(31.0),
            base_y - sx(32.0),
            sx(62.0),
            sx(21.0),
            Color::RAYWHITE,
        );
        draw.draw_text(
            "RUST",
            center - sx(24.0),
            base_y - sx(30.0),
            sx(17.0),
            Color::BLACK,
        );
    }

    fn draw_menu_title(&self, draw: &mut RaylibDrawHandle, width: i32, height: i32) {
        let title = "KART RACER";
        let title_size = (width as f32 * 0.065).clamp(44.0, 76.0) as i32;
        let title_width = draw.measure_text(title, title_size);
        let title_x = (width - title_width) / 2;
        let title_y = (height as f32 * 0.09) as i32;

        for offset in [8, 4] {
            draw.draw_text(
                title,
                title_x + offset,
                title_y + offset,
                title_size,
                Color::new(30, 45, 58, 220),
            );
        }

        draw.draw_text(title, title_x, title_y, title_size, Color::RAYWHITE);
        draw.draw_text(
            title,
            title_x + 3,
            title_y + 3,
            title_size,
            Color::new(24, 31, 42, 85),
        );
    }

    fn draw_menu_panel(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        kart_color: Color,
        kart_color_name: &str,
        music_enabled: bool,
        sfx_enabled: bool,
        selected_option: usize,
        show_controls: bool,
    ) {
        let panel_w = (width as f32 * 0.38).clamp(390.0, 470.0) as i32;

        let panel_x = width / 2 - panel_w - 36;

        let panel_y = (height as f32 * 0.30) as i32;

        let panel_h = (height as f32 * 0.50).clamp(360.0, 390.0) as i32;

        draw.draw_rectangle(
            panel_x + 8,
            panel_y + 8,
            panel_w,
            panel_h,
            Color::new(12, 20, 25, 190),
        );

        draw.draw_rectangle(
            panel_x,
            panel_y,
            panel_w,
            panel_h,
            Color::new(23, 35, 42, 236),
        );

        draw.draw_rectangle_lines(
            panel_x,
            panel_y,
            panel_w,
            panel_h,
            Color::new(98, 184, 88, 255),
        );

        let item_h = 52;

        let start_y = panel_y + 30;

        self.draw_menu_item(
            draw,
            panel_x + 48,
            start_y,
            panel_w - 76,
            item_h,
            "Iniciar carrera",
            0,
            selected_option == 0,
            Color::YELLOW,
        );

        let color_label = format!("Color: {}", kart_color_name);

        self.draw_menu_item(
            draw,
            panel_x + 48,
            start_y + 58,
            panel_w - 76,
            item_h,
            &color_label,
            1,
            selected_option == 1,
            kart_color,
        );

        self.draw_menu_item(
            draw,
            panel_x + 48,
            start_y + 116,
            panel_w - 76,
            item_h,
            &format!("Musica: {}", if music_enabled { "ON" } else { "OFF" }),
            2,
            selected_option == 2,
            if music_enabled {
                Color::YELLOW
            } else {
                Color::LIGHTGRAY
            },
        );

        self.draw_menu_item(
            draw,
            panel_x + 48,
            start_y + 174,
            panel_w - 76,
            item_h,
            &format!("Efectos: {}", if sfx_enabled { "ON" } else { "OFF" }),
            3,
            selected_option == 3,
            if sfx_enabled {
                Color::GREEN
            } else {
                Color::LIGHTGRAY
            },
        );

        self.draw_menu_item(
            draw,
            panel_x + 48,
            start_y + 232,
            panel_w - 76,
            item_h,
            "Controles",
            4,
            selected_option == 4,
            Color::RAYWHITE,
        );

        self.draw_menu_item(
            draw,
            panel_x + 48,
            start_y + 290,
            panel_w - 76,
            item_h,
            "Salir",
            5,
            selected_option == 5,
            Color::RED,
        );

        if selected_option < 6 {
            draw.draw_triangle(
                Vector2::new(
                    (panel_x + 26) as f32,
                    (start_y + selected_option as i32 * 58 + 16) as f32,
                ),
                Vector2::new(
                    (panel_x + 26) as f32,
                    (start_y + selected_option as i32 * 58 + 36) as f32,
                ),
                Vector2::new(
                    (panel_x + 42) as f32,
                    (start_y + selected_option as i32 * 58 + 26) as f32,
                ),
                Color::YELLOW,
            );
        }

        let hint_y = panel_y + panel_h + 20;

        draw.draw_rectangle(panel_x, hint_y, panel_w, 44, Color::new(23, 35, 42, 230));

        draw.draw_rectangle_lines(panel_x, hint_y, panel_w, 44, Color::new(98, 184, 88, 255));

        draw.draw_text(
            "W/S menu  -  A/D cambiar  -  ENTER aceptar",
            panel_x + 22,
            hint_y + 13,
            20,
            Color::RAYWHITE,
        );

        if show_controls {
            self.draw_controls_card(draw, width, height);
        }
    }

    fn draw_track_select_screen(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        selected_track: usize,
        track_count: usize,
        track_select_option: usize,
        track_name: &str,
        kart_color: Color,
        vehicle_index: usize,
        vehicle_name: &str,
    ) {
        self.draw_menu_scene(draw, width, height);

        let title = "SELECCIONA PISTA";
        let title_size = (width as f32 * 0.048).clamp(36.0, 56.0) as i32;
        let title_width = draw.measure_text(title, title_size);
        draw.draw_text(
            title,
            (width - title_width) / 2,
            (height as f32 * 0.08) as i32,
            title_size,
            Color::RAYWHITE,
        );

        let card_w = (width as f32 * 0.32).clamp(280.0, 380.0) as i32;
        let card_h = (height as f32 * 0.42).clamp(250.0, 310.0) as i32;
        let gap = 34;
        let total_w = card_w * track_count as i32 + gap * (track_count as i32 - 1);
        let start_x = (width - total_w) / 2;
        let y = (height as f32 * 0.25) as i32;

        for track in 0..track_count {
            let x = start_x + track as i32 * (card_w + gap);
            self.draw_track_card(
                draw,
                x,
                y,
                card_w,
                card_h,
                track,
                selected_track == track,
                track_select_option == 0,
            );
        }

        let selected = format!("{} listo en {}", vehicle_name, track_name);
        let selected_size = 24;
        let selected_width = draw.measure_text(&selected, selected_size);
        draw.draw_text(
            &selected,
            (width - selected_width) / 2,
            y + card_h + 24,
            selected_size,
            Color::YELLOW,
        );

        let vehicle_y = y + card_h + 62;
        let vehicle_w = 330;
        let vehicle_x = (width - vehicle_w) / 2;
        let vehicle_selected = track_select_option == 1;

        draw.draw_rectangle(
            vehicle_x,
            vehicle_y,
            vehicle_w,
            58,
            Color::new(23, 35, 42, 238),
        );
        draw.draw_rectangle_lines(
            vehicle_x,
            vehicle_y,
            vehicle_w,
            58,
            if vehicle_selected {
                Color::YELLOW
            } else {
                Color::new(98, 184, 88, 255)
            },
        );

        let vehicle_label = format!("< VEHICULO: {} >", vehicle_name.to_uppercase());
        let vehicle_label_width = draw.measure_text(&vehicle_label, 22);
        draw.draw_text(
            &vehicle_label,
            vehicle_x + (vehicle_w - vehicle_label_width) / 2,
            vehicle_y + 18,
            22,
            if vehicle_selected {
                Color::YELLOW
            } else {
                Color::RAYWHITE
            },
        );

        let hint = "W/S elegir fila  -  A/D cambiar  -  ENTER correr  -  BACKSPACE menu";
        let hint_size = 20;
        let hint_width = draw.measure_text(hint, hint_size);
        draw.draw_rectangle(
            (width - hint_width) / 2 - 22,
            height - 62,
            hint_width + 44,
            42,
            Color::new(23, 35, 42, 230),
        );
        draw.draw_rectangle_lines(
            (width - hint_width) / 2 - 22,
            height - 62,
            hint_width + 44,
            42,
            Color::new(98, 184, 88, 255),
        );
        draw.draw_text(
            hint,
            (width - hint_width) / 2,
            height - 50,
            hint_size,
            Color::RAYWHITE,
        );

        self.draw_track_vehicle_badge(draw, width, height, kart_color, vehicle_index);
    }

    fn draw_track_card(
        &self,
        draw: &mut RaylibDrawHandle,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        track: usize,
        selected: bool,
        track_focus: bool,
    ) {
        draw.draw_rectangle(x + 8, y + 8, w, h, Color::new(10, 18, 22, 170));
        draw.draw_rectangle(x, y, w, h, Color::new(23, 35, 42, 238));
        draw.draw_rectangle_lines(
            x,
            y,
            w,
            h,
            if selected {
                if track_focus {
                    Color::YELLOW
                } else {
                    Color::new(170, 150, 70, 255)
                }
            } else {
                Color::new(98, 184, 88, 255)
            },
        );

        let preview_x = x + 24;
        let preview_y = y + 24;
        let preview_w = w - 48;
        let preview_h = h - 92;

        draw.draw_rectangle(
            preview_x,
            preview_y,
            preview_w,
            preview_h,
            if track == 1 {
                Color::new(47, 52, 62, 255)
            } else {
                Color::new(64, 154, 72, 255)
            },
        );

        if track == 1 {
            self.draw_city_track_preview(draw, preview_x, preview_y, preview_w, preview_h);
        } else {
            self.draw_garden_track_preview(draw, preview_x, preview_y, preview_w, preview_h);
        }

        let name = if track == 1 {
            "GRAN PREMIO METRO"
        } else {
            "JARDIN RUST"
        };
        let name_size = 22;
        let name_width = draw.measure_text(name, name_size);
        draw.draw_text(
            name,
            x + (w - name_width) / 2,
            y + h - 52,
            name_size,
            Color::RAYWHITE,
        );

        if selected {
            draw.draw_text("LISTO", x + 20, y + h - 27, 18, Color::YELLOW);
        }
    }

    fn draw_garden_track_preview(
        &self,
        draw: &mut RaylibDrawHandle,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) {
        let road = Color::new(194, 171, 128, 255);
        draw.draw_rectangle(x + w / 8, y + h / 5, w * 3 / 4, h / 5, road);
        draw.draw_rectangle(x + w * 5 / 8, y + h / 5, w / 5, h * 3 / 5, road);
        draw.draw_rectangle(x + w / 8, y + h * 3 / 5, w * 3 / 4, h / 5, road);
        draw.draw_rectangle(x + w / 8, y + h / 5, w / 5, h * 3 / 5, road);
        draw.draw_circle(
            x + w / 2,
            y + h / 2,
            h as f32 * 0.18,
            Color::new(95, 175, 95, 255),
        );
        draw.draw_circle(
            x + w / 2,
            y + h / 2,
            h as f32 * 0.10,
            Color::new(76, 194, 232, 255),
        );
    }

    fn draw_city_track_preview(&self, draw: &mut RaylibDrawHandle, x: i32, y: i32, w: i32, h: i32) {
        let road = Color::new(58, 61, 69, 255);
        let curb = Color::new(245, 205, 55, 255);

        draw.draw_rectangle(x + 18, y + 28, w - 36, 34, road);
        draw.draw_rectangle(x + w - 72, y + 28, 34, h - 62, road);
        draw.draw_rectangle(x + 18, y + h - 64, w - 36, 34, road);
        draw.draw_rectangle(x + 18, y + 62, 34, h - 92, road);
        draw.draw_rectangle(x + w / 4, y + h / 2 - 22, w / 2, 34, road);

        for i in 0..5 {
            draw.draw_rectangle(
                x + 34 + i * 42,
                y + 12,
                28,
                12,
                Color::new(84, 178, 255, 255),
            );
            draw.draw_rectangle(
                x + 44 + i * 36,
                y + h - 24,
                22,
                12,
                Color::new(255, 92, 210, 255),
            );
        }

        draw.draw_rectangle(x + 18, y + 28, w - 36, 4, curb);
        draw.draw_rectangle(x + 18, y + h - 34, w - 36, 4, curb);
    }

    fn draw_track_vehicle_badge(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        color: Color,
        vehicle_index: usize,
    ) {
        let badge_w = 190;
        let badge_h = 70;
        let x = width - badge_w - 34;
        let y = height - badge_h - 28;

        draw.draw_rectangle(x, y, badge_w, badge_h, Color::new(23, 35, 42, 230));
        draw.draw_rectangle_lines(x, y, badge_w, badge_h, Color::YELLOW);
        draw.draw_text("VEHICULO", x + 18, y + 12, 18, Color::YELLOW);
        draw.draw_text(
            if vehicle_index % 2 == 1 {
                "MOTO"
            } else {
                "KART"
            },
            x + 18,
            y + 38,
            24,
            color,
        );
    }

    fn draw_menu_item(
        &self,
        draw: &mut RaylibDrawHandle,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        text: &str,
        icon: usize,
        selected: bool,
        accent: Color,
    ) {
        let bg = if selected {
            Color::new(32, 47, 54, 255)
        } else {
            Color::new(20, 31, 37, 220)
        };

        draw.draw_rectangle(x, y, w, h, bg);

        if selected {
            draw.draw_rectangle_lines(x, y, w, h, Color::YELLOW);
        }

        let icon_x = x + 12;

        let icon_y = y + 8;

        draw.draw_rectangle(icon_x, icon_y, 36, 36, Color::new(35, 48, 56, 255));

        draw.draw_rectangle_lines(icon_x, icon_y, 36, 36, Color::new(96, 112, 120, 255));

        match icon {
            0 => self.draw_flag_icon(draw, icon_x + 8, icon_y + 8),

            1 => {
                draw.draw_rectangle(icon_x + 8, icon_y + 11, 20, 14, accent);
                draw.draw_rectangle_lines(icon_x + 8, icon_y + 11, 20, 14, Color::RAYWHITE);
            }

            2 => self.draw_music_icon(draw, icon_x + 9, icon_y + 8, accent),

            3 => self.draw_sfx_icon(draw, icon_x + 8, icon_y + 9, accent),

            4 => self.draw_pad_icon(draw, icon_x + 8, icon_y + 11),

            _ => {
                draw.draw_line_ex(
                    Vector2::new((icon_x + 10) as f32, (icon_y + 10) as f32),
                    Vector2::new((icon_x + 27) as f32, (icon_y + 27) as f32),
                    6.0,
                    accent,
                );
                draw.draw_line_ex(
                    Vector2::new((icon_x + 27) as f32, (icon_y + 10) as f32),
                    Vector2::new((icon_x + 10) as f32, (icon_y + 27) as f32),
                    6.0,
                    accent,
                );
            }
        }

        draw.draw_text(text, x + 64, y + 15, 24, Color::RAYWHITE);
    }

    fn draw_controls_card(&self, draw: &mut RaylibDrawHandle, width: i32, height: i32) {
        let card_w = 360;

        let card_h = 182;

        let x = width - card_w - 42;

        let y = height - card_h - 42;

        draw.draw_rectangle(x + 8, y + 8, card_w, card_h, Color::new(10, 18, 22, 180));

        draw.draw_rectangle(x, y, card_w, card_h, Color::new(23, 35, 42, 238));

        draw.draw_rectangle_lines(x, y, card_w, card_h, Color::YELLOW);

        draw.draw_text("CONTROLES", x + 26, y + 20, 24, Color::YELLOW);

        let lines = [
            "W/S       acelerar / frenar",
            "A/D       girar",
            "Mouse     rotar camara",
            "SPACE     derrape",
            "P         pausa",
        ];

        for (i, line) in lines.iter().enumerate() {
            draw.draw_text(line, x + 28, y + 58 + i as i32 * 22, 18, Color::RAYWHITE);
        }
    }

    fn draw_flag_icon(&self, draw: &mut RaylibDrawHandle, x: i32, y: i32) {
        for row in 0..3 {
            for col in 0..3 {
                let color = if (row + col) % 2 == 0 {
                    Color::RAYWHITE
                } else {
                    Color::BLACK
                };

                draw.draw_rectangle(x + col * 7, y + row * 7, 7, 7, color);
            }
        }

        draw.draw_rectangle(x, y, 3, 27, Color::LIGHTGRAY);
    }

    fn draw_pad_icon(&self, draw: &mut RaylibDrawHandle, x: i32, y: i32) {
        draw.draw_rectangle(x, y + 8, 26, 14, Color::RAYWHITE);

        draw.draw_circle(x + 5, y + 16, 8.0, Color::RAYWHITE);

        draw.draw_circle(x + 22, y + 16, 8.0, Color::RAYWHITE);

        draw.draw_rectangle(x + 4, y + 13, 9, 3, Color::new(35, 48, 56, 255));

        draw.draw_rectangle(x + 7, y + 10, 3, 9, Color::new(35, 48, 56, 255));

        draw.draw_circle(x + 22, y + 13, 2.0, Color::RED);

        draw.draw_circle(x + 18, y + 17, 2.0, Color::BLUE);
    }

    fn draw_music_icon(&self, draw: &mut RaylibDrawHandle, x: i32, y: i32, accent: Color) {
        draw.draw_rectangle(x + 15, y, 5, 24, accent);
        draw.draw_rectangle(x + 19, y, 10, 4, accent);
        draw.draw_circle(x + 10, y + 23, 7.0, accent);
        draw.draw_circle(x + 24, y + 22, 7.0, accent);
    }

    fn draw_sfx_icon(&self, draw: &mut RaylibDrawHandle, x: i32, y: i32, accent: Color) {
        draw.draw_rectangle(x, y + 9, 7, 10, accent);
        draw.draw_triangle(
            Vector2::new((x + 7) as f32, (y + 9) as f32),
            Vector2::new((x + 17) as f32, (y + 3) as f32),
            Vector2::new((x + 17) as f32, (y + 25) as f32),
            accent,
        );

        draw.draw_line(x + 22, y + 9, x + 27, y + 5, Color::RAYWHITE);
        draw.draw_line(x + 22, y + 14, x + 30, y + 14, Color::RAYWHITE);
        draw.draw_line(x + 22, y + 19, x + 27, y + 23, Color::RAYWHITE);
    }

    fn draw_pause_screen(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        selected_option: usize,
        music_enabled: bool,
        sfx_enabled: bool,
    ) {
        draw.draw_rectangle(0, 0, width, height, Color::new(0, 0, 0, 150));

        let title = "PAUSA";

        let title_size = 64;

        let title_width = draw.measure_text(title, title_size);

        draw.draw_text(
            title,
            (width - title_width) / 2,
            height / 2 - 90,
            title_size,
            Color::YELLOW,
        );

        let panel_w = 360;
        let panel_h = 270;
        let panel_x = (width - panel_w) / 2;
        let panel_y = height / 2 - 10;

        draw.draw_rectangle(
            panel_x + 7,
            panel_y + 7,
            panel_w,
            panel_h,
            Color::new(5, 8, 12, 155),
        );
        draw.draw_rectangle(
            panel_x,
            panel_y,
            panel_w,
            panel_h,
            Color::new(20, 31, 37, 235),
        );
        draw.draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, Color::YELLOW);

        let music_label = format!("Musica: {}", if music_enabled { "ON" } else { "OFF" });
        let sfx_label = format!("Efectos: {}", if sfx_enabled { "ON" } else { "OFF" });
        let options = [
            "Continuar",
            music_label.as_str(),
            sfx_label.as_str(),
            "Volver al menu",
        ];

        for (index, option) in options.iter().enumerate() {
            let item_y = panel_y + 22 + index as i32 * 58;
            let selected = selected_option == index;

            draw.draw_rectangle(
                panel_x + 38,
                item_y,
                panel_w - 76,
                44,
                if selected {
                    Color::new(32, 47, 54, 255)
                } else {
                    Color::new(14, 24, 30, 220)
                },
            );

            if selected {
                draw.draw_rectangle_lines(panel_x + 38, item_y, panel_w - 76, 44, Color::YELLOW);
                draw.draw_triangle(
                    Vector2::new((panel_x + 18) as f32, (item_y + 13) as f32),
                    Vector2::new((panel_x + 18) as f32, (item_y + 31) as f32),
                    Vector2::new((panel_x + 32) as f32, (item_y + 22) as f32),
                    Color::YELLOW,
                );
            }

            draw.draw_text(
                option,
                panel_x + 64,
                item_y + 12,
                22,
                if selected {
                    Color::YELLOW
                } else if index == 1 && music_enabled {
                    Color::YELLOW
                } else if index == 2 && sfx_enabled {
                    Color::YELLOW
                } else {
                    Color::RAYWHITE
                },
            );
        }

        let hint = "W/S elegir - ENTER aceptar - R/P continuar - BACKSPACE menu";

        let hint_size = 18;

        let hint_width = draw.measure_text(hint, hint_size);

        draw.draw_text(
            hint,
            (width - hint_width) / 2,
            panel_y + panel_h + 22,
            hint_size,
            Color::LIGHTGRAY,
        );
    }
}

fn normalize_angle(mut angle: f32) -> f32 {
    while angle > std::f32::consts::PI {
        angle -= std::f32::consts::TAU;
    }

    while angle < -std::f32::consts::PI {
        angle += std::f32::consts::TAU;
    }

    angle
}

fn format_time(seconds: f32) -> String {
    let total_millis = (seconds * 1000.0) as u64;

    let minutes = total_millis / 60_000;

    let seconds = (total_millis / 1000) % 60;

    let millis = total_millis % 1000;

    format!("{:02}:{:02}.{:03}", minutes, seconds, millis,)
}
