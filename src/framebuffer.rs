use raylib::prelude::*;

use crate::{
    kart,
    minimap,
    player::Player,
    race::Race,
    raycaster,
    scenery,
};

pub struct Framebuffer {
    width: i32,
    height: i32,
}

impl Framebuffer {
    pub fn new(
        width: i32,
        height: i32,
    ) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn render(
        &self,
        draw: &mut RaylibDrawHandle,
        map: &[Vec<char>],
        player: &Player,
        race: &Race,
        fov: f32,
        number_of_rays: usize,
    ) {
        scenery::draw_sky(
            draw,
            self.width,
            self.height,
        );

        raycaster::draw_floor(
            draw,
            self.width,
            self.height,
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
            self.width,
            self.height,
            &rays,
            fov,
        );

        scenery::draw_scenery(
            draw,
            self.width,
            self.height,
            map,
            player,
            fov,
            &rays,
        );

        minimap::draw_minimap(
            draw,
            self.width,
            map,
            player,
            &rays,
        );

        kart::draw_kart(
            draw,
            self.width,
            self.height,
            player.velocity,
            player.steering,
        );

        self.draw_race_hud(
            draw,
            race,
        );
    }

    fn draw_race_hud(
        &self,
        draw: &mut RaylibDrawHandle,
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
                draw
            );
        }
    }

    fn draw_finish_screen(
        &self,
        draw: &mut RaylibDrawHandle,
    ) {
        draw.draw_rectangle(
            0,
            0,
            self.width,
            self.height,
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
                self.width
                - title_width
            ) / 2,
            self.height / 2 - 65,
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
                self.width
                - subtitle_width
            ) / 2,
            self.height / 2 + 5,
            subtitle_size,
            Color::RAYWHITE,
        );
    }
}