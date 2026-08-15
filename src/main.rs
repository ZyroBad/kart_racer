mod framebuffer;
mod kart;
mod minimap;
mod player;
mod race;
mod raycaster;
mod scenery;

use framebuffer::Framebuffer;
use player::Player;
use race::Race;
use raylib::prelude::*;
use std::f32::consts::PI;

const WINDOW_WIDTH: i32 = 1200;
const WINDOW_HEIGHT: i32 = 720;

const FOV: f32 = PI / 3.0;
const NUMBER_OF_RAYS: usize = 300;

const MAP: [&str; 41] = [
    "#################################################################",
    "#...............................................................#",
    "#...............................................................#",
    "#...............................................................#",
    "#...PPPPPPPPPPOOPPBBPPCCPPPPPPPPPPPPPPPCCPPBBPPOOPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPOPPPPPPPPOPPPBPPPPPPOPPPPPPPPOPPPPPPPPPPPPP...#",
    "#...PPPPPPPPCPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPCPPPPPPPP...#",
    "#...PPPPPPPPPPPPHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPP...HHHHHHH......T.........T......HHHHHHH...PPPPPPP...#",
    "#...PPPPPPPPPPHPFFFPHPPPPP......A......PPPPPHPFFFPHPPPPPPPPPP...#",
    "#...PPPPPPPPPPHPFFFPHPPPPP..PPPPPPPPP..PPPPPHPFFFPHPPPPPPPPPP...#",
    "#...PPPPPPPPPPHPFFFPHPPPPPPPPPPPPPPPPPPPPPPPHPFFFPHPPPPPPPPPP...#",
    "#...PPPOPPPPPPHHHHHHHPPPOPPPPPPPPPPPPPPPOPPPHHHHHHHPPPPPPOPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPP............PPPPPPPPPPPPPPPPPPP............PPPPPPP...#",
    "#...PPPPPPP.HHHHHHHHHH.PPPPPP.......PPPPPP.HHHHHHHHHH.PPPPPPP...#",
    "#...PPPPPPP.H........HTPPPPPPSSSSSSSPPPPPPTH........H.PPPPPPP...#",
    "#...PPPBPPP.H.FFFFFF.H.PPPPPPSWWWWWSPPPPPP.H.FFFFFF.H.PPPBPPP...#",
    "#...PPPBPPP.H.FFFFFF.H.PPAPPPSWWWWWSPPPAPP.H.FFFFFF.H.PPPBPPP...#",
    "#...PPPPPPP.H.FFFFFF.H.PPPPPPSWWWWWSPPPPPP.H.FFFFFF.H.PPPPPPP...#",
    "#...PPPPPPP.H........HTPPPPPPSSSSSSSPPPPPPTH........H.PPPPPPP...#",
    "#...PPPPPPP.HHHHHHHHHH.PPPPPP.......PPPPPP.HHHHHHHHHH.PPPPPPP...#",
    "#...PPPPPPP............PPPPPPPPPPPPPPPPPPP............PPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPHHHHHHHPPPPPPPPPPPPPPPPPPPPPPPHHHHHHHPPPPPPPPPP...#",
    "#...PPPOPPPPPPHPFFFPHPPPOPPPPPPPPPPPPPPPOPPPHPFFFPHPPPPPPOPPP...#",
    "#...PPPPPPPPPPHPFFFPHPPPPP..PPPPPPPPP..PPPPPHPFFFPHPPPPPPPPPP...#",
    "#...PPPPPPPPPPHPFFFPHPPPPP......A......PPPPPHPFFFPHPPPPPPPPPP...#",
    "#...PPPPPPP...HHHHHHH......T.........T......HHHHHHH...PPPPPPP...#",
    "#...PPPMPPPPPPPPPPOOPPBBPPCCPPPPPPPPPCCPPBBPPOOPPPPPPPPPPPPPP...#",
    "#...PPPMPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPMPPPPCPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPCPPPPPPPP...#",
    "#...PPPMPPPPPPPPPPOPPPPPPPPPOPPBBPPPPOPPPPPPPPPOPPPPPPPPPPPPP...#",
    "#...PPPMPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPMPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...............................................................#",
    "#...............................................................#",
    "#...............................................................#",
    "#################################################################"
];

fn main() {
    let map: Vec<Vec<char>> =
        MAP.iter()
            .map(|row| row.chars().collect())
            .collect();

    let mut player =
        Player::new(6.0, 34.0);

    let mut race =
        Race::new();

    let framebuffer =
        Framebuffer::new();

    let (mut rl, thread) =
        raylib::init()
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .title("Kart Racer - Checkpoints y Vueltas")
            .resizable()
            .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let dt =
            rl.get_frame_time()
                .min(0.05);

        if race.finished()
            && (
                rl.is_key_pressed(
                    KeyboardKey::KEY_ENTER
                )
                || rl.is_key_pressed(
                    KeyboardKey::KEY_R
                )
            )
        {
            player =
                Player::new(6.0, 34.0);

            race =
                Race::new();
        }

        if !race.finished() {
            player.update(
                &rl,
                &map,
                dt,
            );

            race.update(
                &player,
                dt,
            );
        }

        // El tamaño real puede cambiar cuando
        // la ventana se maximiza o se redimensiona.
        let screen_width =
            rl.get_screen_width();

        let screen_height =
            rl.get_screen_height();

        let mut draw =
            rl.begin_drawing(
                &thread
            );

        framebuffer.render(
            &mut draw,
            screen_width,
            screen_height,
            &map,
            &player,
            &race,
            FOV,
            NUMBER_OF_RAYS,
        );
    }
}
