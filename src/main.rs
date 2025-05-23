use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "first_space_game".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}