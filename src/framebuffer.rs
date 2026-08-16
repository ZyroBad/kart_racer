use raylib::prelude::*;

use crate::{
    kart,
    minimap,
    player::Player,
    race::Race,
    raycaster,
    scenery,
};

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
        kart_color: Color,
        kart_color_name: &str,
        track_name: &str,
        countdown_timer: Option<f32>,
        show_pause_screen: bool,
        start_menu_option: usize,
        show_controls: bool,
    ) {
        // IMPORTANTE:
        // Limpiamos TODA la ventana cada frame.
        //
        // Esto evita que al maximizar queden partes negras
        // o imágenes viejas del kart en pantalla.
        draw.clear_background(
            Color::BLACK
        );

        if show_start_screen {
            self.draw_start_screen(
                draw,
                width,
                height,
                kart_color,
                kart_color_name,
                track_name,
                start_menu_option,
                show_controls,
            );

            return;
        }

        scenery::draw_sky(
            draw,
            width,
            height,
        );

        raycaster::draw_floor(
            draw,
            width,
            height,
            map,
            player,
            fov,
        );

        let rays =
            raycaster::cast_all_rays(
                map,
                player,
                fov,
                number_of_rays,
            );

        raycaster::draw_walls(
            draw,
            width,
            height,
            &rays,
            fov,
        );

        scenery::draw_scenery(
            draw,
            width,
            height,
            map,
            player,
            fov,
            &rays,
        );

        scenery::draw_checkpoint(
            draw,
            width,
            height,
            player,
            race,
            fov,
            &rays,
        );

        minimap::draw_minimap(
            draw,
            width,
            map,
            player,
            race,
            &rays,
        );

        self.draw_checkpoint_guide(
            draw,
            width,
            player,
            race,
        );

        kart::draw_kart(
            draw,
            width,
            height,
            player.velocity,
            player.steering,
            player.drift,
            player.boost_flash,
            kart_color,
            race.race_time(),
        );

        self.draw_race_hud(
            draw,
            width,
            height,
            player,
            race,
        );

        self.draw_race_event(
            draw,
            width,
            height,
            race,
        );

        if let Some(timer) =
            countdown_timer
        {
            self.draw_countdown(
                draw,
                width,
                height,
                timer,
            );
        }

        if show_pause_screen {
            self.draw_pause_screen(
                draw,
                width,
                height,
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
        draw.draw_rectangle(
            18,
            18,
            300,
            200,
            Color::new(
                15,
                18,
                22,
                190,
            ),
        );

        draw.draw_text(
            &format!(
                "VUELTA {}/{}",
                race.current_lap(),
                race.total_laps(),
            ),
            30,
            28,
            26,
            Color::RAYWHITE,
        );

        if !race.finished() {
            let objective =
                if race.active_checkpoint_label()
                    == "META"
                {
                    "META".to_string()
                } else {
                    format!(
                        "CHECKPOINT {}/{}",
                        race.current_checkpoint(),
                        race.checkpoint_count() - 1,
                    )
                };

            draw.draw_text(
                &objective,
                30,
                62,
                18,
                Color::YELLOW,
            );
        }

        draw.draw_text(
            &format!(
                "VUELTA: {}",
                format_time(
                    race.lap_time()
                ),
            ),
            30,
            92,
            18,
            Color::RAYWHITE,
        );

        draw.draw_text(
            &format!(
                "TOTAL: {}",
                format_time(
                    race.race_time()
                ),
            ),
            30,
            120,
            18,
            Color::LIGHTGRAY,
        );

        let best_text =
            match race.best_lap_time() {
                Some(time) =>
                    format!(
                        "MEJOR: {}",
                        format_time(time)
                    ),

                None =>
                    "MEJOR: --:--.---"
                        .to_string(),
            };

        draw.draw_text(
            &best_text,
            30,
            148,
            18,
            Color::GREEN,
        );

        if let Some(time) =
            race.last_lap_time()
        {
            draw.draw_text(
                &format!(
                    "ULTIMA: {}",
                    format_time(time)
                ),
                30,
                176,
                16,
                Color::ORANGE,
            );
        }

        draw.draw_fps(
            width - 100,
            height - 32,
        );

        self.draw_speedometer(
            draw,
            width,
            height,
            player,
        );

        if player.boost_flash > 0.1 {
            let boost =
                "BOOST";

            let boost_size =
                24;

            let boost_width =
                draw.measure_text(
                    boost,
                    boost_size,
                );

            draw.draw_text(
                boost,
                width - boost_width - 34,
                height - 74,
                boost_size,
                Color::YELLOW,
            );
        }

        if race.finished() {
            self.draw_finish_screen(
                draw,
                width,
                height,
                race,
            );
        }
    }

    fn draw_finish_screen(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        race: &Race,
    ) {
        draw.draw_rectangle(
            0,
            0,
            width,
            height,
            Color::new(
                0,
                0,
                0,
                175,
            ),
        );

        let title =
            "CARRERA COMPLETADA";

        let title_size =
            44;

        let title_width =
            draw.measure_text(
                title,
                title_size,
            );

        draw.draw_text(
            title,
            (
                width
                - title_width
            ) / 2,
            height / 2 - 105,
            title_size,
            Color::YELLOW,
        );

        let total =
            format!(
                "TIEMPO TOTAL: {}",
                format_time(
                    race.race_time()
                )
            );

        let total_size =
            28;

        let total_width =
            draw.measure_text(
                &total,
                total_size,
            );

        draw.draw_text(
            &total,
            (
                width
                - total_width
            ) / 2,
            height / 2 - 25,
            total_size,
            Color::RAYWHITE,
        );

        let best =
            match race.best_lap_time() {
                Some(time) =>
                    format!(
                        "MEJOR VUELTA: {}",
                        format_time(time)
                    ),

                None =>
                    "MEJOR VUELTA: --:--.---"
                        .to_string(),
            };

        let best_size =
            24;

        let best_width =
            draw.measure_text(
                &best,
                best_size,
            );

        draw.draw_text(
            &best,
            (
                width
                - best_width
            ) / 2,
            height / 2 + 25,
            best_size,
            Color::GREEN,
        );

        let restart =
            "ENTER / R PARA JUGAR DE NUEVO   |   BACKSPACE AL MENU";

        let restart_size =
            20;

        let restart_width =
            draw.measure_text(
                restart,
                restart_size,
            );

        draw.draw_text(
            restart,
            (
                width
                - restart_width
            ) / 2,
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
        let panel_width =
            230;

        let panel_height =
            74;

        let x =
            width
            - panel_width
            - 18;

        let y =
            height
            - panel_height
            - 18;

        draw.draw_rectangle(
            x,
            y,
            panel_width,
            panel_height,
            Color::new(
                12,
                15,
                20,
                185,
            ),
        );

        let speed =
            (
                player.velocity.abs()
                * 18.0
            ) as i32;

        draw.draw_text(
            &format!(
                "{} KM/H",
                speed
            ),
            x + 18,
            y + 12,
            26,
            Color::RAYWHITE,
        );

        let bar_width =
            190;

        let fill =
            (
                player.velocity.abs()
                / 9.0
            )
            .clamp(
                0.0,
                1.0,
            );

        draw.draw_rectangle(
            x + 18,
            y + 50,
            bar_width,
            10,
            Color::new(
                35,
                42,
                50,
                255,
            ),
        );

        draw.draw_rectangle(
            x + 18,
            y + 50,
            (
                bar_width as f32
                * fill
            ) as i32,
            10,
            if player.boost_flash > 0.1 {
                Color::YELLOW
            } else {
                Color::GREEN
            },
        );
    }

    fn draw_race_event(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        race: &Race,
    ) {
        let Some(text) =
            race.event_text()
        else {
            return;
        };

        let timer =
            race.event_timer();

        if timer <= 0.0 {
            return;
        }

        let size =
            (
                34.0
                + timer.min(0.4)
                    * 18.0
            ) as i32;

        let text_width =
            draw.measure_text(
                text,
                size,
            );

        let x =
            (
                width
                - text_width
            ) / 2;

        let y =
            height / 2
            - 150;

        draw.draw_rectangle(
            x - 24,
            y - 12,
            text_width + 48,
            size + 24,
            Color::new(
                8,
                10,
                14,
                170,
            ),
        );

        draw.draw_text(
            text,
            x,
            y,
            size,
            Color::YELLOW,
        );
    }

    fn draw_countdown(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        timer: f32,
    ) {
        draw.draw_rectangle(
            0,
            0,
            width,
            height,
            Color::new(
                0,
                0,
                0,
                85,
            ),
        );

        let text =
            if timer > 2.25 {
                "3"
            } else if timer > 1.25 {
                "2"
            } else if timer > 0.25 {
                "1"
            } else {
                "GO!"
            };

        let size =
            if text == "GO!" {
                76
            } else {
                96
            };

        let text_width =
            draw.measure_text(
                text,
                size,
            );

        draw.draw_text(
            text,
            (
                width
                - text_width
            ) / 2,
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
        let Some(checkpoint) =
            race.active_checkpoint()
        else {
            return;
        };

        let dx =
            checkpoint.x
            - player.x;

        let dy =
            checkpoint.y
            - player.y;

        let distance =
            (
                dx * dx
                + dy * dy
            )
            .sqrt();

        let target_angle =
            dy.atan2(dx);

        let relative_angle =
            normalize_angle(
                target_angle
                - player.angle
            );

        let center_x =
            width / 2;

        let top =
            28;

        let direction =
            if relative_angle.abs()
                < 0.20
            {
                "^"
            } else if relative_angle > 0.0 {
                ">"
            } else {
                "<"
            };

        draw.draw_rectangle(
            center_x - 125,
            top,
            250,
            58,
            Color::new(
                10,
                12,
                18,
                185,
            ),
        );

        let accent =
            race.active_checkpoint_color();

        draw.draw_text(
            direction,
            center_x - 10,
            top + 5,
            30,
            accent,
        );

        let text =
            format!(
                "{}: {:.0}m",
                race.active_checkpoint_label(),
                distance * 3.0
            );

        let text_size =
            16;

        let text_width =
            draw.measure_text(
                &text,
                text_size,
            );

        draw.draw_text(
            &text,
            center_x - text_width / 2,
            top + 38,
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
        track_name: &str,
        selected_option: usize,
        show_controls: bool,
    ) {
        self.draw_menu_scene(
            draw,
            width,
            height,
        );

        self.draw_menu_title(
            draw,
            width,
            height,
        );

        self.draw_menu_panel(
            draw,
            width,
            height,
            kart_color,
            kart_color_name,
            track_name,
            selected_option,
            show_controls,
        );

        self.draw_menu_kart_preview(
            draw,
            width,
            height,
            kart_color,
        );
    }

    fn draw_menu_scene(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
    ) {
        draw.clear_background(
            Color::new(
                55,
                166,
                232,
                255,
            ),
        );

        let horizon =
            (height as f32 * 0.55)
                as i32;

        draw.draw_rectangle(
            0,
            horizon - 50,
            width,
            50,
            Color::new(
                22,
                122,
                48,
                255,
            ),
        );

        draw.draw_rectangle(
            0,
            horizon - 18,
            width,
            26,
            Color::new(
                14,
                86,
                34,
                255,
            ),
        );

        draw.draw_rectangle(
            0,
            horizon,
            width,
            height - horizon,
            Color::new(
                55,
                150,
                66,
                255,
            ),
        );

        let road_top_y =
            horizon;

        draw.draw_triangle(
            Vector2::new(
                (width / 2 + 45) as f32,
                road_top_y as f32,
            ),
            Vector2::new(
                (width / 2 + 185) as f32,
                road_top_y as f32,
            ),
            Vector2::new(
                (width / 2 + 310) as f32,
                height as f32,
            ),
            Color::new(
                187,
                161,
                115,
                255,
            ),
        );

        draw.draw_triangle(
            Vector2::new(
                (width / 2 + 45) as f32,
                road_top_y as f32,
            ),
            Vector2::new(
                (width / 2 - 40) as f32,
                height as f32,
            ),
            Vector2::new(
                (width / 2 + 310) as f32,
                height as f32,
            ),
            Color::new(
                194,
                171,
                128,
                255,
            ),
        );

    }

    fn draw_menu_title(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
    ) {
        let title =
            "KART RACER";

        let title_size =
            (
                width as f32
                * 0.065
            )
            .clamp(
                44.0,
                76.0,
            ) as i32;

        let title_width =
            draw.measure_text(
                title,
                title_size,
            );

        let title_x =
            (width - title_width) / 2;

        let title_y =
            (
                height as f32
                * 0.09
            ) as i32;

        for offset in [
            8,
            4,
        ] {
            draw.draw_text(
                title,
                title_x + offset,
                title_y + offset,
                title_size,
                Color::new(
                    30,
                    45,
                    58,
                    220,
                ),
            );
        }

        draw.draw_text(
            title,
            title_x,
            title_y,
            title_size,
            Color::RAYWHITE,
        );

        draw.draw_text(
            title,
            title_x + 3,
            title_y + 3,
            title_size,
            Color::new(
                24,
                31,
                42,
                85,
            ),
        );

    }

    fn draw_menu_panel(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        kart_color: Color,
        kart_color_name: &str,
        _track_name: &str,
        selected_option: usize,
        show_controls: bool,
    ) {
        let panel_w =
            (
                width as f32
                * 0.38
            )
            .clamp(
                390.0,
                470.0,
            ) as i32;

        let panel_x =
            width / 2
            - panel_w
            - 36;

        let panel_y =
            (
                height as f32
                * 0.39
            ) as i32;

        let panel_h =
            (
                height as f32
                * 0.36
            )
            .clamp(
                244.0,
                276.0,
            ) as i32;

        draw.draw_rectangle(
            panel_x + 8,
            panel_y + 8,
            panel_w,
            panel_h,
            Color::new(
                12,
                20,
                25,
                190,
            ),
        );

        draw.draw_rectangle(
            panel_x,
            panel_y,
            panel_w,
            panel_h,
            Color::new(
                23,
                35,
                42,
                236,
            ),
        );

        draw.draw_rectangle_lines(
            panel_x,
            panel_y,
            panel_w,
            panel_h,
            Color::new(
                98,
                184,
                88,
                255,
            ),
        );

        let item_h =
            52;

        let start_y =
            panel_y + 30;

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

        let color_label =
            format!(
                "Color: {}",
                kart_color_name
            );

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
            "Controles",
            2,
            selected_option == 2,
            Color::RAYWHITE,
        );

        self.draw_menu_item(
            draw,
            panel_x + 48,
            start_y + 174,
            panel_w - 76,
            item_h,
            "Salir",
            3,
            selected_option == 3,
            Color::RED,
        );

        if selected_option < 4 {
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

        let hint_y =
            panel_y
            + panel_h
            + 20;

        draw.draw_rectangle(
            panel_x,
            hint_y,
            panel_w,
            44,
            Color::new(
                23,
                35,
                42,
                230,
            ),
        );

        draw.draw_rectangle_lines(
            panel_x,
            hint_y,
            panel_w,
            44,
            Color::new(
                98,
                184,
                88,
                255,
            ),
        );

        draw.draw_text(
            "W/S menu  -  A/D color  -  ENTER aceptar",
            panel_x + 22,
            hint_y + 13,
            20,
            Color::RAYWHITE,
        );

        if show_controls {
            self.draw_controls_card(
                draw,
                width,
                height,
            );
        }
    }

    fn draw_menu_kart_preview(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        kart_color: Color,
    ) {
        let scale =
            (
                width as f32 / 1200.0
            )
            .min(
                height as f32 / 720.0
            )
            .clamp(
                0.95,
                1.22,
            );

        let center =
            width / 2
            + (
                130.0 * scale
            ) as i32;

        let base_y =
            (
                height as f32
                * 0.77
            ) as i32;

        let sx =
            |value: f32| -> i32 {
                (
                    value
                    * scale
                ) as i32
            };

        draw.draw_ellipse(
            center,
            base_y + sx(18.0),
            118.0 * scale,
            24.0 * scale,
            Color::new(
                20,
                20,
                20,
                130,
            ),
        );

        draw.draw_rectangle(
            center - sx(110.0),
            base_y - sx(70.0),
            sx(34.0),
            sx(82.0),
            Color::new(
                23,
                24,
                28,
                255,
            ),
        );

        draw.draw_rectangle(
            center + sx(76.0),
            base_y - sx(70.0),
            sx(34.0),
            sx(82.0),
            Color::new(
                23,
                24,
                28,
                255,
            ),
        );

        draw.draw_rectangle(
            center - sx(94.0),
            base_y - sx(30.0),
            sx(188.0),
            sx(34.0),
            self.menu_shade_color(
                kart_color,
                0.74,
            ),
        );

        draw.draw_rectangle(
            center - sx(76.0),
            base_y - sx(102.0),
            sx(152.0),
            sx(74.0),
            kart_color,
        );

        draw.draw_rectangle(
            center - sx(56.0),
            base_y - sx(132.0),
            sx(112.0),
            sx(38.0),
            self.menu_shade_color(
                kart_color,
                1.12,
            ),
        );

        draw.draw_rectangle(
            center - sx(38.0),
            base_y - sx(148.0),
            sx(76.0),
            sx(50.0),
            Color::new(
                32,
                36,
                42,
                255,
            ),
        );

        draw.draw_circle(
            center,
            base_y - sx(170.0),
            34.0 * scale,
            Color::new(
                245,
                185,
                72,
                255,
            ),
        );

        draw.draw_rectangle(
            center - sx(30.0),
            base_y - sx(196.0),
            sx(60.0),
            sx(24.0),
            self.menu_shade_color(
                kart_color,
                0.92,
            ),
        );

        draw.draw_rectangle(
            center - sx(22.0),
            base_y - sx(174.0),
            sx(44.0),
            sx(10.0),
            Color::SKYBLUE,
        );

        draw.draw_rectangle(
            center - sx(28.0),
            base_y - sx(32.0),
            sx(56.0),
            sx(20.0),
            Color::RAYWHITE,
        );

        draw.draw_text(
            "RUST",
            center - sx(21.0),
            base_y - sx(30.0),
            sx(16.0),
            Color::BLACK,
        );
    }

    fn menu_shade_color(
        &self,
        color: Color,
        factor: f32,
    ) -> Color {
        Color::new(
            (
                color.r as f32
                * factor
            )
            .clamp(
                0.0,
                255.0,
            ) as u8,
            (
                color.g as f32
                * factor
            )
            .clamp(
                0.0,
                255.0,
            ) as u8,
            (
                color.b as f32
                * factor
            )
            .clamp(
                0.0,
                255.0,
            ) as u8,
            color.a,
        )
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
        let bg =
            if selected {
                Color::new(
                    32,
                    47,
                    54,
                    255,
                )
            } else {
                Color::new(
                    20,
                    31,
                    37,
                    220,
                )
            };

        draw.draw_rectangle(
            x,
            y,
            w,
            h,
            bg,
        );

        if selected {
            draw.draw_rectangle_lines(
                x,
                y,
                w,
                h,
                Color::YELLOW,
            );
        }

        let icon_x =
            x + 12;

        let icon_y =
            y + 8;

        draw.draw_rectangle(
            icon_x,
            icon_y,
            36,
            36,
            Color::new(
                35,
                48,
                56,
                255,
            ),
        );

        draw.draw_rectangle_lines(
            icon_x,
            icon_y,
            36,
            36,
            Color::new(
                96,
                112,
                120,
                255,
            ),
        );

        match icon {
            0 => self.draw_flag_icon(
                draw,
                icon_x + 8,
                icon_y + 8,
            ),

            1 => {
                draw.draw_rectangle(
                    icon_x + 8,
                    icon_y + 11,
                    20,
                    14,
                    accent,
                );
                draw.draw_rectangle_lines(
                    icon_x + 8,
                    icon_y + 11,
                    20,
                    14,
                    Color::RAYWHITE,
                );
            }

            2 => self.draw_pad_icon(
                draw,
                icon_x + 8,
                icon_y + 11,
            ),

            _ => {
                draw.draw_line_ex(
                    Vector2::new(
                        (icon_x + 10) as f32,
                        (icon_y + 10) as f32,
                    ),
                    Vector2::new(
                        (icon_x + 27) as f32,
                        (icon_y + 27) as f32,
                    ),
                    6.0,
                    accent,
                );
                draw.draw_line_ex(
                    Vector2::new(
                        (icon_x + 27) as f32,
                        (icon_y + 10) as f32,
                    ),
                    Vector2::new(
                        (icon_x + 10) as f32,
                        (icon_y + 27) as f32,
                    ),
                    6.0,
                    accent,
                );
            }
        }

        draw.draw_text(
            text,
            x + 64,
            y + 15,
            24,
            Color::RAYWHITE,
        );
    }

    fn draw_controls_card(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
    ) {
        let card_w =
            360;

        let card_h =
            182;

        let x =
            width - card_w - 42;

        let y =
            height - card_h - 42;

        draw.draw_rectangle(
            x + 8,
            y + 8,
            card_w,
            card_h,
            Color::new(
                10,
                18,
                22,
                180,
            ),
        );

        draw.draw_rectangle(
            x,
            y,
            card_w,
            card_h,
            Color::new(
                23,
                35,
                42,
                238,
            ),
        );

        draw.draw_rectangle_lines(
            x,
            y,
            card_w,
            card_h,
            Color::YELLOW,
        );

        draw.draw_text(
            "CONTROLES",
            x + 26,
            y + 20,
            24,
            Color::YELLOW,
        );

        let lines =
            [
                "W/S       acelerar / frenar",
                "A/D       girar",
                "Mouse     rotar camara",
                "SPACE     derrape",
                "P         pausa",
            ];

        for (i, line) in lines.iter().enumerate() {
            draw.draw_text(
                line,
                x + 28,
                y + 58 + i as i32 * 22,
                18,
                Color::RAYWHITE,
            );
        }
    }

    fn draw_flag_icon(
        &self,
        draw: &mut RaylibDrawHandle,
        x: i32,
        y: i32,
    ) {
        for row in 0..3 {
            for col in 0..3 {
                let color =
                    if (
                        row
                        + col
                    )
                        % 2
                        == 0
                    {
                        Color::RAYWHITE
                    } else {
                        Color::BLACK
                    };

                draw.draw_rectangle(
                    x + col * 7,
                    y + row * 7,
                    7,
                    7,
                    color,
                );
            }
        }

        draw.draw_rectangle(
            x,
            y,
            3,
            27,
            Color::LIGHTGRAY,
        );
    }

    fn draw_pad_icon(
        &self,
        draw: &mut RaylibDrawHandle,
        x: i32,
        y: i32,
    ) {
        draw.draw_rectangle(
            x,
            y + 8,
            26,
            14,
            Color::RAYWHITE,
        );

        draw.draw_circle(
            x + 5,
            y + 16,
            8.0,
            Color::RAYWHITE,
        );

        draw.draw_circle(
            x + 22,
            y + 16,
            8.0,
            Color::RAYWHITE,
        );

        draw.draw_rectangle(
            x + 4,
            y + 13,
            9,
            3,
            Color::new(
                35,
                48,
                56,
                255,
            ),
        );

        draw.draw_rectangle(
            x + 7,
            y + 10,
            3,
            9,
            Color::new(
                35,
                48,
                56,
                255,
            ),
        );

        draw.draw_circle(
            x + 22,
            y + 13,
            2.0,
            Color::RED,
        );

        draw.draw_circle(
            x + 18,
            y + 17,
            2.0,
            Color::BLUE,
        );
    }

    fn draw_pause_screen(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
    ) {
        draw.draw_rectangle(
            0,
            0,
            width,
            height,
            Color::new(
                0,
                0,
                0,
                150,
            ),
        );

        let title =
            "PAUSA";

        let title_size =
            64;

        let title_width =
            draw.measure_text(
                title,
                title_size,
            );

        draw.draw_text(
            title,
            (
                width
                - title_width
            ) / 2,
            height / 2 - 90,
            title_size,
            Color::YELLOW,
        );

        let resume =
            "ENTER / R PARA CONTINUAR";

        let resume_size =
            24;

        let resume_width =
            draw.measure_text(
                resume,
                resume_size,
            );

        draw.draw_text(
            resume,
            (
                width
                - resume_width
            ) / 2,
            height / 2 + 5,
            resume_size,
            Color::RAYWHITE,
        );

        let hint =
            "P tambien reanuda";

        let hint_size =
            18;

        let hint_width =
            draw.measure_text(
                hint,
                hint_size,
            );

        draw.draw_text(
            hint,
            (
                width
                - hint_width
            ) / 2,
            height / 2 + 45,
            hint_size,
            Color::LIGHTGRAY,
        );
    }
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

fn format_time(
    seconds: f32,
) -> String {
    let total_millis =
        (seconds * 1000.0)
            as u64;

    let minutes =
        total_millis
        / 60_000;

    let seconds =
        (
            total_millis
            / 1000
        ) % 60;

    let millis =
        total_millis
        % 1000;

    format!(
        "{:02}:{:02}.{:03}",
        minutes,
        seconds,
        millis,
    )
}
