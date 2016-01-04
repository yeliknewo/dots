extern crate piston_window;

use piston_window::*;

use std::sync::{Mutex, Arc, RwLock};
use std::thread::{self, JoinHandle};

const WIDTH: usize = 1920 / 16;
const HEIGHT: usize = 1080 / 16;

use std::collections::VecDeque;

struct World {
    dot_color: Color,
    dot_data: Arc<RwLock<Vec<Vec<f32>>>>,
    events: Arc<Mutex<VecDeque<WorldEvents>>>,
    threads: Vec<JoinHandle<()>>,
}

impl World {
    fn new(color: Color) -> World {
        let mut vec: Vec<Vec<f32>> = vec!();
        for _ in 0..HEIGHT {
            let mut vec_y: Vec<f32> = vec!();
            for _ in 0..WIDTH {
                vec_y.push(0.0);
            }
            vec.push(vec_y);
        }
        World {
            dot_color: color,
            dot_data: Arc::new(RwLock::new(vec)),
            events: Arc::new(Mutex::new(VecDeque::new())),
            threads: vec!(),
        }
    }

    fn update_red(&mut self) {
        for y in 0..HEIGHT {
            let dots = self.dot_data.clone();
            let world_events = self.events.clone();
            self.threads.push(thread::spawn(move || {
                for x in 0..WIDTH {
                    let dots = dots.read().expect("Dots Lock Error");
                    let mut world_events = world_events.lock().expect("World Events Lock Error");
                    let scan_sum = scan_loop(&*dots, x, y, 1);
                    if scan_sum > 5.0 {
                        world_events.push_back(WorldEvents::Add(x, y, -0.01));
                    } else if scan_sum > 4.0 {
                        world_events.push_back(WorldEvents::Mul(x, y, 0.9 * (y % 8) as f32));
                    } else if scan_sum > 3.9 {
                        world_events.push_back(WorldEvents::Mul(x, y, 1.1 * ((x % 4) + (y % 4)) as f32));
                    } else if scan_sum > 3.0 {
                        world_events.push_back(WorldEvents::Mul(x, y, 0.9 * (x % 8) as f32));
                    } else {
                        world_events.push_back(WorldEvents::Add(x, y, 0.01 * (1.0 + (x % 2) as f32)));
                    }
                }
            }));
        }
    }

    fn update_green(&mut self) {
        for y in 0..HEIGHT {
            let dots = self.dot_data.clone();
            let world_events = self.events.clone();
            self.threads.push(thread::spawn(move || {
                for x in 0..WIDTH {
                    let dots = dots.read().expect("Dots Lock Error");
                    let mut world_events = world_events.lock().expect("World Events Lock Error");
                    let scan_sum = scan_loop(&*dots, x, y, 1);
                    if scan_sum > 5.0 {
                        world_events.push_back(WorldEvents::Add(x, y, -0.01));
                    } else if scan_sum > 4.0 {
                        world_events.push_back(WorldEvents::Mul(x, y, 0.9 * (y % 8) as f32));
                    } else if scan_sum > 3.9 {
                        world_events.push_back(WorldEvents::Mul(x, y, 1.1 * ((x % 8) + (y % 8)) as f32));
                    } else if scan_sum > 3.0 {
                        world_events.push_back(WorldEvents::Mul(x, y, 0.9 * (x % 8) as f32));
                    } else {
                        world_events.push_back(WorldEvents::Add(x, y, 0.01 * (1.0 + (x % 2) as f32)));
                    }
                }
            }));
        }
    }

    fn update_blue(&mut self) {
        for y in 0..HEIGHT {
            let dots = self.dot_data.clone();
            let world_events = self.events.clone();
            self.threads.push(thread::spawn(move || {
                for x in 0..WIDTH {
                    let dots = dots.read().expect("Dots Lock Error");
                    let mut world_events = world_events.lock().expect("World Events Lock Error");
                    let scan_sum = scan_loop(&dots, x, y, 1);
                    if scan_sum > 5.0 {
                        world_events.push_back(WorldEvents::Add(x, y, -0.01));
                    } else if scan_sum > 4.0 {
                        world_events.push_back(WorldEvents::Mul(x, y, 0.9 * (y % 4) as f32));
                    } else if scan_sum > 3.9 {
                        world_events.push_back(WorldEvents::Mul(x, y, 1.1 * ((x % 8) + (y % 8)) as f32));
                    } else if scan_sum > 3.0 {
                        world_events.push_back(WorldEvents::Mul(x, y, 0.9 * (x % 4) as f32));
                    } else {
                        world_events.push_back(WorldEvents::Add(x, y, 0.01 * (1.0 + (x % 2) as f32)));
                    }
                }
            }));
        }

    }

    fn update(&mut self) {
        match self.dot_color {
            Color::Red => self.update_red(),
            Color::Green => self.update_green(),
            Color::Blue => self.update_blue(),
        }
        loop {
            match self.threads.pop() {
                Some(thread) => thread.join().expect("Thread Join Error"),
                None => break,
            }
        }
        loop {
            let dots = self.dot_data.clone();
            match self.events.lock().expect("World Events Lock Error").pop_front() {
                Some(event) => match event {
                    WorldEvents::Add(x, y, val) => dots.write().expect("Unable to Write to Dot Data")[y][x] += val,
                    WorldEvents::Mul(x, y, val) => dots.write().expect("Unable to Write to Dot Data")[y][x] *= val,
                    WorldEvents::Set(x, y, val) => dots.write().expect("Unable to Write to Dot Data")[y][x] = val,
                },
                None => break,
            }
        }
    }

    fn get_data_at(&self, x: usize, y: usize) -> f32 {
        let dots = self.dot_data.clone();
        let val = dots.read().expect("Unable to Write to Dot Data")[y][x];
        return val
    }

    fn get_color(&self) -> Color {
        self.dot_color
    }
}

#[derive(Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue
}

fn main() {
    let window: PistonWindow = WindowSettings::new("Hello Piston!", [1920, 1080])
        .exit_on_esc(true)
        .build()
        .expect("Error building PistonWindow");

    let mut worlds = vec![World::new(Color::Red), World::new(Color::Green), World::new(Color::Blue)];

    let dot_width = window.size().width as f64 / WIDTH as f64;
    let dot_height = window.size().height as f64 / HEIGHT as f64;

    for e in window {
        for mut world in &mut worlds {
            world.update();
        }
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let mut red = 0.0;
                    let mut green = 0.0;
                    let mut blue = 0.0;
                    for world in &worlds {
                        match world.get_color() {
                            Color::Red => red = world.get_data_at(x, y),
                            Color::Green => green = world.get_data_at(x, y),
                            Color::Blue => blue = world.get_data_at(x, y),
                        }
                    }
                    rectangle(
                        [red, green, blue, 1.0],
                        [x as f64 * dot_width as f64, y as f64 * dot_height, dot_width, dot_height],
                        c.transform,
                        g
                    );
                }
            }
        })
    }
}

fn scan_loop(dots: &Vec<Vec<f32>>, start_x: usize, start_y: usize, range: isize) -> f32 {
    let mut sum = 0.0;
    for y in -range..range {
        let mut dy = y + start_y as isize;
        if dy < 0 {
            dy += HEIGHT as isize;
        }
        if dy >= HEIGHT as isize {
            dy -= HEIGHT as isize;
        }
        for x in -range..range {
            let mut dx = x + start_x as isize;
            if dx < 0 {
                dx += WIDTH as isize;
            }
            if dx >= WIDTH as isize {
                dx -= WIDTH as isize;
            }
            sum += dots[dy as usize][dx as usize];
        }
    }
    sum
}

enum WorldEvents {
    Add(usize, usize, f32),
    Mul(usize, usize, f32),
    Set(usize, usize, f32),
}
