use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use snake::{
    candy::Candy, cell::Cell, direction::Direction, drawable::Drawable, frame::Frame,
    position::Position, renderer::Renderer, size::Size, snake::Snake, vector::Vector,
};

/**
 * https://github.com/baurst/rs_snake/blob/master/src/snake.rs
 * https://github.com/CleanCut/rusty_time/blob/main/src/lib.rs
 * https://github.com/CleanCut/invaders/blob/main/src/player.rs
 */

fn main() {
    let size = Size {
        width: 40,
        height: 40,
    };

    // Setup terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Hide).unwrap();

    let (render_tx, render_rx) = mpsc::channel();
    let mut instant = Instant::now();
    let mut snake = Snake::new(
        Position { x: 10, y: 10 },
        Vector::from_direction(Direction::Left),
        5,
    );
    let mut candy = Candy::new();
    let render_handle = thread::spawn(move || {
        let mut renderer = Renderer::new(size);
        let mut stdout = io::stdout();

        while let Ok(frame) = render_rx.recv() {
            renderer.render(&mut stdout, frame);
        }
    });

    'gameloop: loop {
        let mut frame = Frame::new(size, Cell::Empty);
        let delta = instant.elapsed();
        instant = Instant::now();

        while event::poll(Duration::default()).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Left => snake.direction = Vector::from_direction(Direction::Left),
                    KeyCode::Right => snake.direction = Vector::from_direction(Direction::Right),
                    KeyCode::Up => snake.direction = Vector::from_direction(Direction::Up),
                    KeyCode::Down => snake.direction = Vector::from_direction(Direction::Down),
                    KeyCode::Esc => break 'gameloop,
                    _ => {}
                }
            }
        }

        if !snake.tick(delta) {
            break 'gameloop;
        }

        if !frame.is_free_position(snake.position) {
            break 'gameloop;
        }

        if !candy.tick(delta) {
            loop {
                let position = frame.random_position();

                if !snake.collision(position) && snake.position != position {
                    candy.set_position(position);

                    break;
                }
            }
        }

        if candy.eat(snake.position) {
            snake.grow(3);
            candy.reset();
        }

        snake.draw(&mut frame);
        candy.draw(&mut frame);

        render_tx.send(frame).unwrap();

        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();

    // Cleanup terminal
    stdout.execute(LeaveAlternateScreen).unwrap();
    stdout.execute(Show).unwrap();
    terminal::disable_raw_mode().unwrap();
}
