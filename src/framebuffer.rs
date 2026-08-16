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
        countdown_timer: Option<f32>,
        show_pause_screen: bool,
    ) {
        // IMPORTANTE:
        // Limpiamos TODA la ventana cada frame.
        //
        // Esto evita que al maximizar queden partes negras
        // o imágenes viejas del kart en pantalla.
        draw.clear_background(
            Color::BLACK
        );

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

        if show_start_screen {
            self.draw_start_screen(
                draw,
                width,
                height,
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
            draw.draw_text(
                &format!(
                    "CHECKPOINT {}/{}",
                    race.current_checkpoint() + 1,
                    race.checkpoint_count(),
                ),
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

        draw.draw_text(
            direction,
            center_x - 10,
            top + 5,
            30,
            Color::YELLOW,
        );

        let text =
            format!(
                "META: {:.0}m",
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
                165,
            ),
        );

        let title =
            "RUST KART RACE";

        let title_size =
            54;

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
            height / 2 - 130,
            title_size,
            Color::YELLOW,
        );

        let subtitle =
            "3 vueltas - usa los boosts - evita salirte de pista";

        let subtitle_size =
            22;

        let subtitle_width =
            draw.measure_text(
                subtitle,
                subtitle_size,
            );

        draw.draw_text(
            subtitle,
            (
                width
                - subtitle_width
            ) / 2,
            height / 2 - 55,
            subtitle_size,
            Color::RAYWHITE,
        );

        let controls =
            "W/S acelerar/frenar   A/D girar   Mouse camara   Space derrape";

        let controls_size =
            18;

        let controls_width =
            draw.measure_text(
                controls,
                controls_size,
            );

        draw.draw_text(
            controls,
            (
                width
                - controls_width
            ) / 2,
            height / 2 + 4,
            controls_size,
            Color::LIGHTGRAY,
        );

        let start =
            "ENTER / SPACE PARA EMPEZAR";

        let start_size =
            26;

        let start_width =
            draw.measure_text(
                start,
                start_size,
            );

        draw.draw_text(
            start,
            (
                width
                - start_width
            ) / 2,
            height / 2 + 78,
            start_size,
            Color::GREEN,
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
