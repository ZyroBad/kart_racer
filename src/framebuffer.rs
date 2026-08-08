use raylib::prelude::*;

use crate::{
    kart,
    minimap,
    player::Player,
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

        draw.draw_text(
            "W/S acelerar-reversa | A/D girar",
            18,
            18,
            21,
            Color::RAYWHITE,
        );
    }
}