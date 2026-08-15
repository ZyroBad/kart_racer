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
            270,
            92,
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
                64,
                19,
                Color::YELLOW,
            );
        }

        if race.finished() {
            self.draw_finish_screen(
                draw,
                width,
                height,
            );
        }
    }

    fn draw_finish_screen(
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
                170,
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
            height / 2 - 65,
            title_size,
            Color::YELLOW,
        );

        let subtitle =
            "3 vueltas completadas";

        let subtitle_size =
            26;

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
            height / 2 + 5,
            subtitle_size,
            Color::RAYWHITE,
        );
    }
}