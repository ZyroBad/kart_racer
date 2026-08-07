mod framebuffer;
mod player;

use framebuffer::Framebuffer;
use player::Player;
use raylib::prelude::*;
use std::f32::consts::PI;

const WINDOW_WIDTH: i32 = 1200;
const WINDOW_HEIGHT: i32 = 720;

const FOV: f32 = PI / 3.0;
const NUMBER_OF_RAYS: usize = 240;

const MAP: [&str; 21] = [
    "#############################",
    "#.............#.............#",
    "#.#####.#####.#.#####.#####.#",
    "#.#...#.....#.#.#.....#...#.#",
    "#.#.R.#####.#.#.#.#####.G.#.#",
    "#.#.........#...#.........#.#",
    "#.#########.#####.#########.#",
    "#.........#.......#.........#",
    "#########.#.#####.#.#########",
    "#.........#.#...#.#.........#",
    "#.#########.#.Y.#.#########.#",
    "#...........#...#...........#",
    "#.#######.####.####.#######.#",
    "#.#.....#......#......#.....#",
    "#.#.###.###########.###.###.#",
    "#...#.................#.....#",
    "###.#.#####.#####.#####.###.#",
    "#...#.....#.....#.....#.....#",
    "#.#######.#####.#####.#####.#",
    "#...........................#",
    "#############################",
];

fn main() {
    let map: Vec<Vec<char>> =
        MAP.iter()
            .map(|row| row.chars().collect())
            .collect();

    // Empieza en un corredor abierto mirando hacia la derecha.
    let mut player = Player::new(2.5, 1.5);

    let framebuffer =
        Framebuffer::new(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        );

    let (mut rl, thread) =
        raylib::init()
            .size(
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
            .title("Rusty Maze Kart - movimiento")
            .resizable()
            .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // Evita saltos enormes si la ventana se congela un momento.
        let delta_time =
            rl.get_frame_time().min(0.05);

        // Toda la lógica del carro vive en player.rs.
        player.update(
            &rl,
            &map,
            delta_time,
        );

        let mut draw =
            rl.begin_drawing(&thread);

        framebuffer.render(
            &mut draw,
            &map,
            &player,
            FOV,
            NUMBER_OF_RAYS,
        );
    }
}
