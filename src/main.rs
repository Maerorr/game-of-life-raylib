use raylib::prelude::*;

mod game_of_life;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1000, 1000)
        .title("Game of Life")
        .build();

    let mut game = game_of_life::GameOfLife::new(100, 100);
    rl.set_target_fps(144);

    // start measturing time
    let mut start = std::time::Instant::now();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        
        game.draw(&mut d);
        if start.elapsed().as_millis() > 64 {
            game.update();
            start = std::time::Instant::now();
        }
    }
}