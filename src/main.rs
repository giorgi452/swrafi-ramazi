use platform::Platform;
use player::Player;
use rand::Rng;
use raylib::prelude::*;

pub mod platform;
pub mod player;

fn main() {
    // Setup Window
    // ------------
    let (mut rl, thread) = raylib::init().title("Swrafi Ramazi").build();
    // ------------

    // Toggle Fullscreen and Focus
    // --------------------------
    rl.toggle_fullscreen();
    rl.toggle_borderless_windowed();
    rl.set_window_focused();
    rl.hide_cursor();
    rl.set_target_fps(240);
    // --------------------------

    // Create Player Instance
    // ----------------------
    let mut player = Player::new(
        5,
        get_monitor_height(get_current_monitor()) - 70,
        0.0,
        false,
    );
    // ----------------------

    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..=get_monitor_width(get_current_monitor()));

    let mut platform = Platform::new(
        random_number,
        get_monitor_height(get_current_monitor()) - 80,
        50,
        10,
        true,
    );

    // While window isn't closed
    // -------------------------
    while !rl.window_should_close() {
        // Begin Drawing
        // -------------
        let mut d = rl.begin_drawing(&thread);
        // -------------

        // Set Background
        // --------------
        d.clear_background(Color::BLACK);
        // --------------

        player.draw(&mut d);
        {
            player.movements(&mut d);
            platform.move_left()
        }

        if platform.show {
            platform.draw(&mut d);
        }
    }
    // -------------------------
}
