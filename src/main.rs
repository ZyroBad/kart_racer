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
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPP.............................................PPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPP.......T.T.......PPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPP.................PPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPP.............PPPPPPPPPPPPPPPPPPP.............PPPPPP...#",
    "#...PPPPPP.............PPPPPPPPPPPPPPPPPPP.............PPPPPP...#",
    "#...PPPPPP..HHHHHHHHH..PPPPP.........PPPPP..HHHHHHHHH..PPPPPP...#",
    "#...PPPPPP..H.......H..PPPPP.SSSSSSS.PPPPP..H.......H..PPPPPP...#",
    "#...PPPPPP..H.FFFFF.H..PPPPP.SWWWWWS.PPPPP..H.FFFFF.H..PPPPPP...#",
    "#...PPPPPP.TH.FFFFF.H..PPPPP.SWWWWWS.PPPPP..H.FFFFF.HT.PPPPPP...#",
    "#...PPPPPP..H.FFFFF.H..PPPPP.SWWWWWS.PPPPP..H.FFFFF.H..PPPPPP...#",
    "#...PPPPPP..H.......H..PPPPP.SSSSSSS.PPPPP..H.......H..PPPPPP...#",
    "#...PPPPPP..HHHHHHHHH..PPPPP.........PPPPP..HHHHHHHHH..PPPPPP...#",
    "#...PPPPPP.............PPPPPPPPPPPPPPPPPPP.............PPPPPP...#",
    "#...PPPPPP.............PPPPPPPPPPPPPPPPPPP.............PPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPP...............PPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPPPPPPPPPPPPPPPPP......T.T......PPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPPPP.............................................PPPPPP...#",
    "#...PPPMPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPMPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
    "#...PPPMPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...#",
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
        Framebuffer::new(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        );

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

        if !race.finished() {
            player.update(
                &rl,
                &map,
                dt,
            );

            race.update(
                &player
            );
        }

        let mut draw =
            rl.begin_drawing(
                &thread
            );

        framebuffer.render(
            &mut draw,
            &map,
            &player,
            &race,
            FOV,
            NUMBER_OF_RAYS,
        );
    }
}