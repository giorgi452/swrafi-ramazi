use platform::Platform;
use player::Player;
use rand::Rng;
use raylib::prelude::*;

pub mod platform;
pub mod player;

pub enum GameState {
    STARTMENU,
    ACTGAME,
    GAMEOVER,
}

fn main() {
    let mut game_state = GameState::ACTGAME;
    let (mut rl, thread) = raylib::init().title("Swrafi Ramazi").build();

    rl.toggle_fullscreen();
    rl.toggle_borderless_windowed();
    rl.set_window_focused();
    rl.hide_cursor();
    rl.set_target_fps(240);

    let mut player = Player::new(
        5,
        get_monitor_height(get_current_monitor()) - 70,
        0.0,
        false,
    );

    let mut rng = rand::thread_rng();
    let mut platforms: Vec<Platform> = (0..20)
        .map(|_| {
            let x = rng.gen_range(0..get_monitor_width(get_current_monitor()));
            let y = rng.gen_range(
                get_monitor_height(get_current_monitor()) - 150
                    ..get_monitor_height(get_current_monitor()) - 5,
            );
            Platform::new(x, y, 20, 10, true)
        })
        .collect();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        match game_state {
            GameState::STARTMENU => {}
            GameState::ACTGAME => {
                player.draw(&mut d);

                {
                    for platform in &mut platforms {
                        if platform.show {
                            platform.draw(&mut d);
                        }
                    }
                }

                {
                    player.movements(&mut d, &mut platforms, &mut game_state);
                }
            }
            GameState::GAMEOVER => {
                d.draw_text("Game Over", 50, 50, 20, Color::RED);
            }
        }
    }
    // -------------------------
}
