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

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameState {
    Start,
    Countdown,
    Racing,
    Paused,
    Finished,
}

const MAP: [&str; 41] = [
    "#################################################################",
    "#..................G.....G.....G.....G.....G....................#",
    "#..................G.....G.....G.....G.....G....................#",
    "#...............................................................#",
    "#...KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK...#",
    "#...KPPPPPPPKPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPPPPOPPPPPPPPPPPRPPPRPPPPPPPPPPPOPPPPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPBPPLLLPPPLLCPPPLLLPPPLLCPPPLLLPPPLBPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPKPPPPPPPK...#",
    "#...KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK...#",
    "#...KPPPPPPPK.......................................KPPPPPPPK...#",
    "#...KPPPPPPPK.......................................KPPPPPPPK...#",
    "#...KPPPLPPPK....HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH....KPPPLPPPK...#",
    "#...KPPPLPPPK....H.............................H....KPPPLPPPK...#",
    "#...KPPPLPPPK....H.............................H....KPPPLPPPK...#",
    "#...KPPPPPPPK....H..T...T...............T...T..H....KPPPPPPPK...#",
    "#...KPPPPPPPK....H....F...................F....H....KPPPPPPPK...#",
    "#...KPPPPPPPK....H.........SS.......SS.........H....KPPPPPPPK...#",
    "#...KPPPLPPPK....H.........SSWWWWWWWSS.........H....KPPRRPPPK...#",
    "#...KPPPLPPPK....H.........SSWWWWWWWSS.........H....KPPRRPPPK...#",
    "#...KPPPRRPPK....H.........SSWWWWWWWSS.........H....KPPPLPPPK...#",
    "#...KPPPRRPPK....H.........SSWWWWWWWSS.........H....KPPPPPPPK...#",
    "#...KPPPPPPPK....H.........SSWWWWWWWSS.........H....KPPPPPPPK...#",
    "#...KPPPPPPPK....H.........SS.......SS.........H....KPPPPPPPK...#",
    "#...KPPPLPPPK....H....F...................F....H....KPPPLPPPK...#",
    "#...KPPPLPPPK....H..T...T...............T...T..H....KPPPLPPPK...#",
    "#...KPPPLPPPK....H.............................H....KPPPLPPPK...#",
    "#...KPPPPPPPK....H.............................H....KPPPPPPPK...#",
    "#...KPPPPPPPK....HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH....KPPPPPPPK...#",
    "#...KPPPPPPPK.......................................KPPPPPPPK...#",
    "#...KPPPPPPPK.......................................KPPPPPPPK...#",
    "#...MMMMMKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK...#",
    "#...MMMMMPPPKPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPKPPPPPPPK...#",
    "#...MMMMMPPPKPPBPPPPPPPPPPCPPPPPPPPPPPCPPPPPPPPPPBPPKPPPPPPPK...#",
    "#...MMMMMPPPKPPPPPOLLPPPLLLPPPRLLPRPLLLPPPLLLPOPLLPPKPPPPPPPK...#",
    "#...MMMMMPPPKPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPKPPPPPPPK...#",
    "#...MMMMMKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK...#",
    "#...............................................................#",
    "#..................G.....G.....G.....G.....G....................#",
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

    let mut game_state =
        GameState::Start;

    let mut countdown_timer =
        0.0_f32;

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

        match game_state {
            GameState::Start => {
                if rl.is_key_pressed(
                    KeyboardKey::KEY_ENTER
                )
                    || rl.is_key_pressed(
                        KeyboardKey::KEY_SPACE
                    )
                {
                    player =
                        Player::new(6.0, 34.0);

                    race =
                        Race::new();

                    countdown_timer =
                        3.25;

                    game_state =
                        GameState::Countdown;
                }
            }

            GameState::Countdown => {
                countdown_timer -= dt;

                if countdown_timer <= 0.0 {
                    countdown_timer = 0.0;

                    game_state =
                        GameState::Racing;
                }
            }

            GameState::Racing => {
                if rl.is_key_pressed(
                    KeyboardKey::KEY_P
                ) {
                    game_state =
                        GameState::Paused;
                } else {

                    let mouse_delta =
                        rl.get_mouse_delta();

                    player.update(
                        &rl,
                        &map,
                        dt,
                        mouse_delta.x,
                    );

                    race.update(
                        &player,
                        dt,
                    );

                    if race.finished() {
                        game_state =
                            GameState::Finished;
                    }
                }
            }

            GameState::Paused => {
                if rl.is_key_pressed(
                    KeyboardKey::KEY_ENTER
                )
                    || rl.is_key_pressed(
                        KeyboardKey::KEY_R
                    )
                    || rl.is_key_pressed(
                        KeyboardKey::KEY_P
                    )
                {
                    game_state =
                        GameState::Racing;
                }
            }

            GameState::Finished => {
                if rl.is_key_pressed(
                    KeyboardKey::KEY_ENTER
                )
                    || rl.is_key_pressed(
                        KeyboardKey::KEY_R
                    )
                {
                    player =
                        Player::new(6.0, 34.0);

                    race =
                        Race::new();

                    countdown_timer =
                        3.25;

                    game_state =
                        GameState::Countdown;
                }

                if rl.is_key_pressed(
                    KeyboardKey::KEY_BACKSPACE
                ) {
                    player =
                        Player::new(6.0, 34.0);

                    race =
                        Race::new();

                    game_state =
                        GameState::Start;
                }
            }
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
            game_state == GameState::Start,
            if game_state == GameState::Countdown {
                Some(countdown_timer)
            } else {
                None
            },
            game_state == GameState::Paused,
        );
    }
}
