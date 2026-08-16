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
const KART_COLOR_COUNT: usize = 5;
const START_MENU_COUNT: usize = 4;

const MENU_START: usize = 0;
const MENU_COLOR: usize = 1;
const MENU_CONTROLS: usize = 2;
const MENU_EXIT: usize = 3;

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
    "#..................G...........G...........G....................#",
    "#...............................G...............................#",
    "#..............G.................................G..............#",
    "#...KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK...#",
    "#...KPPPPPPPKPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPPPPOPPPPPPPPPPPRPPPRPPPPPPPPPPPOPPPPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPBPPLLLPPPLLCPPPLLLPPPLLCPPPLLLPPPLBPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPKPPPPPPPK...#",
    "#...KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK...#",
    "#...KPPYPPPPK.......................................KPPPPYPPK...#",
    "#...KPPPPPPPK....O..............Y..............O....KPPPPPPPK...#",
    "#...KPPPLPPPK..B.HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH.B..KPPPLPPPK...#",
    "#...KPPPLPPPK....H.............................H....KPPPLPPPK...#",
    "#...KPPPLPPPK....H.............................H....KPPPLPPPK...#",
    "#...KPPPPPPPK....H..T...T...............T...T..H....KPPPPPPPK...#",
    "#...KPPPPPPPK....H....F...................F....H....KPPPPPPPK...#",
    "#...KPPPPPPPK....H.........SS.......SS.........H....KPPPPPPPK...#",
    "#...KPPPLPPPK....H.........SSWWWWWWWSS.........H....KPPRRPPPK...#",
    "#...KPPPLPPPK....H.........SSWWWWWWWSS.........H....KPPRRPPPK...#",
    "#...KPPPRRPPK....H.........SSWWWQWWWSS.........H....KPPPLPPPK...#",
    "#...KPPPRRPPK....H.........SSWWWWWWWSS.........H....KPPPPPPPK...#",
    "#...KPPPPPPPK....H.........SSWWWWWWWSS.........H....KPPPPPPPK...#",
    "#...KPPPPPPPK....H.........SS.......SS.........H....KPPPPPPPK...#",
    "#...KPPPLPPPK....H....F...................F....H....KPPPLPPPK...#",
    "#...KPPPLPPPK....H..T...T...............T...T..H....KPPPLPPPK...#",
    "#...KPPPLPPPK....H.............................H....KPPPLPPPK...#",
    "#...KPPYPPPPK....H.............................H....KPPPPYPPK...#",
    "#...KPPPPPPPK....HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH....KPPPPPPPK...#",
    "#...KPPPPPPPK....O..............Y..............O....KPPPPPPPK...#",
    "#...KPPPPPPPK..B.................................B..KPPPPPPPK...#",
    "#...KKKKKKKKKKKKKKMMMMNKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK...#",
    "#...KPPPPPPPKPPPPPMMMMMPPPPPPPPPPPPPPPPPPPPPPPPPPPPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPBPPMMMMMPPPCPPPPPPPPPPPCPPPPPPPPPPBPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPPPPMMMNMPLLLPPPRLLPRPLLLPPPLLLPOPLLPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPPPPMMMMMPPPPPPPPPPPPPPPPPPPPPPPPPPPPPKPPPPPPPK...#",
    "#...KKKKKKKKKKKKKKMMMMMKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK...#",
    "#..............G.................................G..............#",
    "#..................G...........G...........G....................#",
    "#...............................................................#",
    "#################################################################"
];

fn main() {
    let map: Vec<Vec<char>> =
        MAP.iter()
            .map(|row| row.chars().collect())
            .collect();

    let mut player =
        Player::new(15.0, 34.0);

    let mut race =
        Race::new();

    let mut game_state =
        GameState::Start;

    let mut countdown_timer =
        0.0_f32;

    let mut selected_kart_color =
        0_usize;

    let mut start_menu_option =
        MENU_START;

    let mut show_controls =
        false;

    let framebuffer =
        Framebuffer::new();

    let (mut rl, thread) =
        raylib::init()
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .title("Kart Racer - Checkpoints y Vueltas")
            .resizable()
            .build();

    rl.set_target_fps(60);

    'game_loop: while !rl.window_should_close() {
        let dt =
            rl.get_frame_time()
                .min(0.05);

        match game_state {
            GameState::Start => {
                if rl.is_key_pressed(
                    KeyboardKey::KEY_DOWN
                )
                    || rl.is_key_pressed(
                        KeyboardKey::KEY_S
                    )
                {
                    start_menu_option =
                        (
                            start_menu_option
                            + 1
                        )
                        % START_MENU_COUNT;

                    show_controls =
                        false;
                }

                if rl.is_key_pressed(
                    KeyboardKey::KEY_UP
                )
                    || rl.is_key_pressed(
                        KeyboardKey::KEY_W
                    )
                {
                    start_menu_option =
                        (
                            start_menu_option
                            + START_MENU_COUNT
                            - 1
                        )
                        % START_MENU_COUNT;

                    show_controls =
                        false;
                }

                if rl.is_key_pressed(
                    KeyboardKey::KEY_RIGHT
                )
                    || rl.is_key_pressed(
                        KeyboardKey::KEY_D
                    )
                {
                    if start_menu_option
                        == MENU_COLOR
                    {
                        selected_kart_color =
                            (
                                selected_kart_color
                                + 1
                            )
                            % KART_COLOR_COUNT;
                    }
                }

                if rl.is_key_pressed(
                    KeyboardKey::KEY_LEFT
                )
                    || rl.is_key_pressed(
                        KeyboardKey::KEY_A
                    )
                {
                    if start_menu_option
                        == MENU_COLOR
                    {
                        selected_kart_color =
                            (
                                selected_kart_color
                                + KART_COLOR_COUNT
                                - 1
                            )
                            % KART_COLOR_COUNT;
                    }
                }

                if rl.is_key_pressed(
                    KeyboardKey::KEY_ENTER
                )
                    || rl.is_key_pressed(
                        KeyboardKey::KEY_SPACE
                    )
                {
                    match start_menu_option {
                        MENU_START => {
                            player =
                                Player::new(15.0, 34.0);

                            race =
                                Race::new();

                            countdown_timer =
                                3.25;

                            show_controls =
                                false;

                            game_state =
                                GameState::Countdown;
                        }

                        MENU_COLOR => {
                            selected_kart_color =
                                (
                                    selected_kart_color
                                    + 1
                                )
                                % KART_COLOR_COUNT;
                        }

                        MENU_CONTROLS => {
                            show_controls =
                                !show_controls;
                        }

                        MENU_EXIT => {
                            break 'game_loop;
                        }

                        _ => {}
                    }
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
                        Player::new(15.0, 34.0);

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
                        Player::new(15.0, 34.0);

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
            kart_color(
                selected_kart_color
            ),
            kart_color_name(
                selected_kart_color
            ),
            "Circuito Rust",
            if game_state == GameState::Countdown {
                Some(countdown_timer)
            } else {
                None
            },
            game_state == GameState::Paused,
            start_menu_option,
            show_controls,
        );
    }
}

fn kart_color(
    index: usize,
) -> Color {
    match index % KART_COLOR_COUNT {
        0 =>
            Color::new(
                220,
                43,
                42,
                255,
            ),

        1 =>
            Color::new(
                42,
                118,
                235,
                255,
            ),

        2 =>
            Color::new(
                45,
                185,
                85,
                255,
            ),

        3 =>
            Color::new(
                245,
                195,
                45,
                255,
            ),

        _ =>
            Color::new(
                190,
                75,
                220,
                255,
            ),
    }
}

fn kart_color_name(
    index: usize,
) -> &'static str {
    match index % KART_COLOR_COUNT {
        0 => "Rojo",
        1 => "Azul",
        2 => "Verde",
        3 => "Amarillo",
        _ => "Morado",
    }
}
