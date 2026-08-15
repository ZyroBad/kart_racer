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

        kart::draw_kart(
            draw,
            width,
            height,
            player.velocity,
            player.steering,
            player.drift,
            race.race_time(),
        );

        self.draw_race_hud(
            draw,
            width,
            height,
            race,
        );
    }

    fn draw_race_hud(
        &self,
        draw: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
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
            "ENTER / R PARA JUGAR DE NUEVO";

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
