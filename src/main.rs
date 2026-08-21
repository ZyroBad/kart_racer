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
use std::path::Path;

const WINDOW_WIDTH: i32 = 1200;
const WINDOW_HEIGHT: i32 = 720;

const FOV: f32 = PI / 3.0;
const NUMBER_OF_RAYS: usize = 300;
const KART_COLOR_COUNT: usize = 5;
const TRACK_COUNT: usize = 2;
const VEHICLE_COUNT: usize = 2;
const START_MENU_COUNT: usize = 6;
const TRACK_SELECT_COUNT: usize = 2;

const MENU_START: usize = 0;
const MENU_COLOR: usize = 1;
const MENU_MUSIC: usize = 2;
const MENU_SFX: usize = 3;
const MENU_CONTROLS: usize = 4;
const MENU_EXIT: usize = 5;

const TRACK_SELECT_TRACK: usize = 0;
const TRACK_SELECT_VEHICLE: usize = 1;
const PAUSE_MENU_COUNT: usize = 4;
const PAUSE_CONTINUE: usize = 0;
const PAUSE_MUSIC: usize = 1;
const PAUSE_SFX: usize = 2;
const PAUSE_BACK_TO_MENU: usize = 3;

const MUSIC_FILES: [&str; 3] = [
    "assets/audio/music/besame_mucho.wav",
    "assets/audio/music/besame_mucho.ogg",
    "assets/audio/music/vuelve.ogg",
];
const ENGINE_SOUND_FILE: &str = "assets/audio/sfx/engine.wav";
const BOOST_SOUND_FILE: &str = "assets/audio/sfx/boost.wav";
const CHECKPOINT_SOUND_FILE: &str = "assets/audio/sfx/checkpoint.wav";

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameState {
    Start,
    TrackSelect,
    Countdown,
    Racing,
    Paused,
    Finished,
}

const MAP_GARDEN: [&str; 41] = [
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
    "#...KPPPPPPPKPPPPPMMMMMPRRRPPPPPPPPPPPPPPPPPPPPPPPPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPBPPMMMMMPPPCPPPPPPPPPPPCPPPPPPPPPPBPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPPPPMMMNMPLLLPPPRLLPRPLLLPPPLLLPOPLLPPKPPPPPPPK...#",
    "#...KPPPPPPPKPPPPPMMMMMPPPPPPPPPPPPPPPPPPPPPRRRPPPPPKPPPPPPPK...#",
    "#...KKKKKKKKKKKKKKMMMMMKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK...#",
    "#..............G.................................G..............#",
    "#..................G...........G...........G....................#",
    "#...............................................................#",
    "#################################################################",
];

fn main() {
    let mut selected_track = 0_usize;

    let mut map = build_track_map(selected_track);

    let mut player = spawn_player(selected_track);

    let mut race = Race::new(selected_track);

    let mut game_state = GameState::Start;

    let mut countdown_timer = 0.0_f32;

    let mut selected_kart_color = 0_usize;

    let mut selected_vehicle = 0_usize;

    let mut start_menu_option = MENU_START;

    let mut track_select_option = TRACK_SELECT_TRACK;

    let mut pause_menu_option = PAUSE_CONTINUE;

    let mut show_controls = false;

    let mut music_enabled = true;

    let mut sfx_enabled = true;

    let framebuffer = Framebuffer::new();

    let audio = raylib::audio::RaylibAudio::init_audio_device().ok();
    let mut music_tracks = Vec::new();
    let mut engine_sound = None;
    let mut boost_sound = None;
    let mut checkpoint_sound = None;

    if let Some(audio_device) = audio.as_ref() {
        for path in MUSIC_FILES {
            if Path::new(path).exists() {
                if let Ok(mut music) = audio_device.new_music(path) {
                    music.set_looping(false);
                    music.set_volume(0.46);
                    music_tracks.push(music);
                }
            }
        }

        if Path::new(ENGINE_SOUND_FILE).exists() {
            engine_sound = audio_device.new_sound(ENGINE_SOUND_FILE).ok();
            if let Some(sound) = engine_sound.as_ref() {
                sound.set_volume(0.28);
            }
        }

        if Path::new(BOOST_SOUND_FILE).exists() {
            boost_sound = audio_device.new_sound(BOOST_SOUND_FILE).ok();
            if let Some(sound) = boost_sound.as_ref() {
                sound.set_volume(0.42);
            }
        }

        if Path::new(CHECKPOINT_SOUND_FILE).exists() {
            checkpoint_sound = audio_device.new_sound(CHECKPOINT_SOUND_FILE).ok();
            if let Some(sound) = checkpoint_sound.as_ref() {
                sound.set_volume(0.45);
            }
        }
    }

    let mut current_music_track = 0_usize;
    let mut last_boost_flash = 0.0_f32;
    let mut last_checkpoint = race.current_checkpoint();

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Kart Racer - Checkpoints y Vueltas")
        .resizable()
        .build();

    rl.set_target_fps(60);

    'game_loop: while !rl.window_should_close() {
        let dt = rl.get_frame_time().min(0.05);

        match game_state {
            GameState::Start => {
                if rl.is_key_pressed(KeyboardKey::KEY_DOWN) || rl.is_key_pressed(KeyboardKey::KEY_S)
                {
                    start_menu_option = (start_menu_option + 1) % START_MENU_COUNT;

                    show_controls = false;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_W) {
                    start_menu_option =
                        (start_menu_option + START_MENU_COUNT - 1) % START_MENU_COUNT;

                    show_controls = false;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_RIGHT)
                    || rl.is_key_pressed(KeyboardKey::KEY_D)
                {
                    if start_menu_option == MENU_COLOR {
                        selected_kart_color = (selected_kart_color + 1) % KART_COLOR_COUNT;
                    } else if start_menu_option == MENU_MUSIC {
                        music_enabled = !music_enabled;

                        if music_enabled {
                            restart_music(&music_tracks, &mut current_music_track);
                        } else {
                            stop_music(&music_tracks);
                        }
                    } else if start_menu_option == MENU_SFX {
                        sfx_enabled = !sfx_enabled;
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_LEFT) || rl.is_key_pressed(KeyboardKey::KEY_A)
                {
                    if start_menu_option == MENU_COLOR {
                        selected_kart_color =
                            (selected_kart_color + KART_COLOR_COUNT - 1) % KART_COLOR_COUNT;
                    } else if start_menu_option == MENU_MUSIC {
                        music_enabled = !music_enabled;

                        if music_enabled {
                            restart_music(&music_tracks, &mut current_music_track);
                        } else {
                            stop_music(&music_tracks);
                        }
                    } else if start_menu_option == MENU_SFX {
                        sfx_enabled = !sfx_enabled;
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
                    || rl.is_key_pressed(KeyboardKey::KEY_SPACE)
                {
                    match start_menu_option {
                        MENU_START => {
                            show_controls = false;

                            game_state = GameState::TrackSelect;
                        }

                        MENU_COLOR => {
                            selected_kart_color = (selected_kart_color + 1) % KART_COLOR_COUNT;
                        }

                        MENU_MUSIC => {
                            music_enabled = !music_enabled;

                            if music_enabled {
                                restart_music(&music_tracks, &mut current_music_track);
                            } else {
                                stop_music(&music_tracks);
                            }
                        }

                        MENU_SFX => {
                            sfx_enabled = !sfx_enabled;
                        }

                        MENU_CONTROLS => {
                            show_controls = !show_controls;
                        }

                        MENU_EXIT => {
                            break 'game_loop;
                        }

                        _ => {}
                    }
                }
            }

            GameState::TrackSelect => {
                if rl.is_key_pressed(KeyboardKey::KEY_DOWN) || rl.is_key_pressed(KeyboardKey::KEY_S)
                {
                    track_select_option = (track_select_option + 1) % TRACK_SELECT_COUNT;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_W) {
                    track_select_option =
                        (track_select_option + TRACK_SELECT_COUNT - 1) % TRACK_SELECT_COUNT;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_RIGHT)
                    || rl.is_key_pressed(KeyboardKey::KEY_D)
                {
                    if track_select_option == TRACK_SELECT_TRACK {
                        selected_track = (selected_track + 1) % TRACK_COUNT;
                    } else if track_select_option == TRACK_SELECT_VEHICLE {
                        selected_vehicle = (selected_vehicle + 1) % VEHICLE_COUNT;
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_LEFT) || rl.is_key_pressed(KeyboardKey::KEY_A)
                {
                    if track_select_option == TRACK_SELECT_TRACK {
                        selected_track = (selected_track + TRACK_COUNT - 1) % TRACK_COUNT;
                    } else if track_select_option == TRACK_SELECT_VEHICLE {
                        selected_vehicle = (selected_vehicle + VEHICLE_COUNT - 1) % VEHICLE_COUNT;
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
                    || rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
                {
                    game_state = GameState::Start;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
                    || rl.is_key_pressed(KeyboardKey::KEY_SPACE)
                {
                    map = build_track_map(selected_track);

                    player = spawn_player(selected_track);

                    race = Race::new(selected_track);

                    countdown_timer = 3.25;

                    game_state = GameState::Countdown;
                }
            }

            GameState::Countdown => {
                countdown_timer -= dt;

                if countdown_timer <= 0.0 {
                    countdown_timer = 0.0;

                    game_state = GameState::Racing;
                }
            }

            GameState::Racing => {
                if rl.is_key_pressed(KeyboardKey::KEY_P) {
                    pause_menu_option = PAUSE_CONTINUE;
                    game_state = GameState::Paused;
                } else {
                    let mouse_delta = rl.get_mouse_delta();

                    player.update(&rl, &map, dt, mouse_delta.x);

                    race.update(&player, dt);

                    if sfx_enabled && race.current_checkpoint() != last_checkpoint {
                        if let Some(sound) = checkpoint_sound.as_ref() {
                            sound.play();
                        }
                    }

                    if race.finished() {
                        game_state = GameState::Finished;
                    }
                }
            }

            GameState::Paused => {
                if rl.is_key_pressed(KeyboardKey::KEY_DOWN) || rl.is_key_pressed(KeyboardKey::KEY_S)
                {
                    pause_menu_option = (pause_menu_option + 1) % PAUSE_MENU_COUNT;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_W) {
                    pause_menu_option =
                        (pause_menu_option + PAUSE_MENU_COUNT - 1) % PAUSE_MENU_COUNT;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_P) || rl.is_key_pressed(KeyboardKey::KEY_R) {
                    game_state = GameState::Racing;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
                    game_state = GameState::Start;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
                    || rl.is_key_pressed(KeyboardKey::KEY_SPACE)
                {
                    match pause_menu_option {
                        PAUSE_CONTINUE => {
                            game_state = GameState::Racing;
                        }

                        PAUSE_MUSIC => {
                            music_enabled = !music_enabled;

                            if music_enabled {
                                restart_music(&music_tracks, &mut current_music_track);
                            } else {
                                stop_music(&music_tracks);
                            }
                        }

                        PAUSE_SFX => {
                            sfx_enabled = !sfx_enabled;
                        }

                        PAUSE_BACK_TO_MENU => {
                            game_state = GameState::Start;
                        }

                        _ => {}
                    }
                }
            }

            GameState::Finished => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
                    || rl.is_key_pressed(KeyboardKey::KEY_R)
                {
                    map = build_track_map(selected_track);

                    player = spawn_player(selected_track);

                    race = Race::new(selected_track);

                    countdown_timer = 3.25;

                    game_state = GameState::Countdown;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
                    map = build_track_map(selected_track);

                    player = spawn_player(selected_track);

                    race = Race::new(selected_track);

                    game_state = GameState::Start;
                }
            }
        }

        update_audio(
            &music_tracks,
            &mut current_music_track,
            music_enabled,
            sfx_enabled,
            &engine_sound,
            &boost_sound,
            player.velocity,
            player.boost_flash,
            last_boost_flash,
            game_state == GameState::Racing,
        );

        last_boost_flash = player.boost_flash;
        last_checkpoint = race.current_checkpoint();

        // El tamaño real puede cambiar cuando
        // la ventana se maximiza o se redimensiona.
        let screen_width = rl.get_screen_width();

        let screen_height = rl.get_screen_height();

        let mut draw = rl.begin_drawing(&thread);

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
            game_state == GameState::TrackSelect,
            kart_color(selected_kart_color),
            kart_color_name(selected_kart_color),
            track_name(selected_track),
            selected_track,
            TRACK_COUNT,
            track_select_option,
            selected_vehicle,
            vehicle_name(selected_vehicle),
            music_enabled,
            sfx_enabled,
            if game_state == GameState::Countdown {
                Some(countdown_timer)
            } else {
                None
            },
            game_state == GameState::Paused,
            pause_menu_option,
            start_menu_option,
            show_controls,
        );
    }
}

fn update_audio(
    music_tracks: &[raylib::audio::Music],
    current_music_track: &mut usize,
    music_enabled: bool,
    sfx_enabled: bool,
    engine_sound: &Option<raylib::audio::Sound>,
    boost_sound: &Option<raylib::audio::Sound>,
    velocity: f32,
    boost_flash: f32,
    last_boost_flash: f32,
    racing: bool,
) {
    if !music_tracks.is_empty() {
        for music in music_tracks {
            music.update_stream();
        }

        let active_index = *current_music_track % music_tracks.len();
        let active_music = &music_tracks[active_index];

        if music_enabled {
            if !active_music.is_stream_playing() {
                active_music.play_stream();
            }

            let length = active_music.get_time_length();
            let played = active_music.get_time_played();

            if length > 0.1 && played >= length - 0.08 {
                active_music.stop_stream();
                *current_music_track = (*current_music_track + 1) % music_tracks.len();
                music_tracks[*current_music_track].play_stream();
            }
        } else if active_music.is_stream_playing() {
            active_music.pause_stream();
        }
    }

    if !sfx_enabled || !racing {
        return;
    }

    if velocity.abs() > 0.65 {
        if let Some(sound) = engine_sound {
            if !sound.is_playing() {
                sound.play();
            }
        }
    }

    if boost_flash > 0.85 && last_boost_flash <= 0.10 {
        if let Some(sound) = boost_sound {
            sound.play();
        }
    }
}

fn stop_music(music_tracks: &[raylib::audio::Music]) {
    for music in music_tracks {
        music.stop_stream();
        music.seek_stream(0.0);
    }
}

fn restart_music(music_tracks: &[raylib::audio::Music], current_music_track: &mut usize) {
    if music_tracks.is_empty() {
        return;
    }

    stop_music(music_tracks);
    *current_music_track = 0;
    music_tracks[0].play_stream();
}

fn kart_color(index: usize) -> Color {
    match index % KART_COLOR_COUNT {
        0 => Color::new(220, 43, 42, 255),

        1 => Color::new(42, 118, 235, 255),

        2 => Color::new(45, 185, 85, 255),

        3 => Color::new(245, 195, 45, 255),

        _ => Color::new(190, 75, 220, 255),
    }
}

fn build_track_map(track_index: usize) -> Vec<Vec<char>> {
    match track_index % TRACK_COUNT {
        1 => build_city_map(),
        _ => MAP_GARDEN.iter().map(|row| row.chars().collect()).collect(),
    }
}

fn spawn_player(track_index: usize) -> Player {
    match track_index % TRACK_COUNT {
        1 => Player::new(17.0, 34.0),
        _ => Player::new(15.0, 34.0),
    }
}

fn track_name(index: usize) -> &'static str {
    match index % TRACK_COUNT {
        1 => "Gran Premio Metro",
        _ => "Jardin Rust",
    }
}

fn vehicle_name(index: usize) -> &'static str {
    match index % VEHICLE_COUNT {
        1 => "Moto",
        _ => "Kart",
    }
}

fn build_city_map() -> Vec<Vec<char>> {
    let mut map = vec![vec!['U'; 65]; 41];

    for x in 0..65 {
        map[0][x] = '#';
        map[40][x] = '#';
    }

    for row in &mut map {
        row[0] = '#';
        row[64] = '#';
    }

    fill_rect(&mut map, 3, 4, 61, 9, 'K');
    fill_rect(&mut map, 4, 5, 60, 8, 'P');

    fill_rect(&mut map, 52, 8, 61, 35, 'K');
    fill_rect(&mut map, 53, 9, 60, 34, 'P');

    fill_rect(&mut map, 3, 31, 61, 37, 'K');
    fill_rect(&mut map, 4, 32, 60, 36, 'P');

    fill_rect(&mut map, 3, 8, 12, 37, 'K');
    fill_rect(&mut map, 4, 9, 11, 36, 'P');

    fill_rect(&mut map, 11, 13, 27, 18, 'K');
    fill_rect(&mut map, 12, 14, 26, 17, 'P');

    fill_rect(&mut map, 24, 17, 35, 22, 'K');
    fill_rect(&mut map, 25, 18, 34, 21, 'P');

    fill_rect(&mut map, 33, 22, 51, 27, 'K');
    fill_rect(&mut map, 34, 23, 50, 26, 'P');

    fill_rect(&mut map, 43, 10, 58, 15, 'K');
    fill_rect(&mut map, 44, 11, 57, 14, 'P');

    fill_rect(&mut map, 18, 23, 28, 34, 'K');
    fill_rect(&mut map, 19, 24, 27, 33, 'P');

    fill_rect(&mut map, 15, 11, 21, 12, 'D');
    fill_rect(&mut map, 30, 10, 39, 15, 'D');
    fill_rect(&mut map, 37, 28, 46, 31, 'D');
    fill_rect(&mut map, 14, 20, 20, 22, 'D');
    fill_rect(&mut map, 43, 17, 49, 20, 'D');

    fill_rect(&mut map, 19, 32, 22, 36, 'M');
    map[34][22] = 'N';
    map[33][22] = 'N';

    write_hline(&mut map, 35, 32, 34, 'R');
    write_hline(&mut map, 11, 47, 49, 'R');
    write_hline(&mut map, 36, 24, 26, 'R');
    write_hline(&mut map, 14, 15, 17, 'R');

    for &(x, y) in &[
        (8, 11),
        (23, 14),
        (29, 19),
        (39, 24),
        (56, 17),
        (55, 29),
        (9, 29),
    ] {
        map[y][x] = 'Y';
    }

    for &(x, y) in &[
        (14, 30),
        (30, 30),
        (49, 30),
        (14, 10),
        (41, 10),
        (50, 16),
        (29, 23),
    ] {
        map[y][x] = 'B';
    }

    for &(x, y) in &[(6, 6), (58, 6), (6, 35), (58, 35), (22, 20), (51, 22)] {
        map[y][x] = 'O';
    }

    for &(x, y) in &[
        (13, 6),
        (28, 6),
        (43, 6),
        (56, 12),
        (56, 25),
        (45, 35),
        (30, 35),
        (14, 35),
        (7, 24),
        (7, 13),
    ] {
        map[y][x] = 'V';
    }

    for &(x, y) in &[(51, 10), (13, 19), (35, 23), (52, 28), (12, 31)] {
        map[y][x] = 'Z';
    }

    for &(x, y) in &[(23, 10), (42, 16), (48, 21), (29, 28), (16, 25)] {
        map[y][x] = 'E';
    }

    map
}

fn fill_rect(map: &mut [Vec<char>], x1: usize, y1: usize, x2: usize, y2: usize, tile: char) {
    for row in map.iter_mut().take(y2 + 1).skip(y1) {
        for cell in row.iter_mut().take(x2 + 1).skip(x1) {
            *cell = tile;
        }
    }
}

fn write_hline(map: &mut [Vec<char>], y: usize, x1: usize, x2: usize, tile: char) {
    for x in x1..=x2 {
        map[y][x] = tile;
    }
}

fn kart_color_name(index: usize) -> &'static str {
    match index % KART_COLOR_COUNT {
        0 => "Rojo",
        1 => "Azul",
        2 => "Verde",
        3 => "Amarillo",
        _ => "Morado",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track_maps_keep_expected_size() {
        for track in 0..TRACK_COUNT {
            let map = build_track_map(track);

            assert_eq!(map.len(), 41);

            for row in map {
                assert_eq!(row.len(), 65);
            }
        }
    }

    #[test]
    fn track_spawns_start_on_drivable_tiles() {
        for track in 0..TRACK_COUNT {
            let map = build_track_map(track);
            let player = spawn_player(track);

            let tile = map[player.y.floor() as usize][player.x.floor() as usize];

            assert!(matches!(tile, 'P' | 'M' | 'R' | 'L' | 'K'));
        }
    }

    #[test]
    fn city_track_is_distinct_and_has_city_tiles() {
        let garden = build_track_map(0);
        let city = build_track_map(1);

        assert_ne!(garden, city);
        assert_eq!(track_name(1), "Gran Premio Metro");
        assert!(city.iter().flatten().any(|tile| *tile == 'D'));
        assert!(city.iter().flatten().any(|tile| *tile == 'U'));
        assert!(city.iter().flatten().any(|tile| *tile == 'V'));
        assert!(city.iter().flatten().any(|tile| *tile == 'Z'));
        assert!(city.iter().flatten().any(|tile| *tile == 'E'));
    }
}
