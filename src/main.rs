use std::{thread, time};

fn main() {
    let rule_string = "B3/S23";

    let mut world: Vec<Vec<bool>> = Vec::new();
    let width: usize = 256;
    let height: usize = 256;

    let mut app = simple::Window::new("Game of Life", 512, 512);

    for y in 0..height {
        world.push(Vec::new());
        for _ in 0..width {
            world[y].push(rand::random());
        }
    }

    // loop {
    //     draw_world_terminal(&world);
    //
    //     world = update_world(&world, rule_string);
    //
    //     thread::sleep(time::Duration::from_millis(100));
    // }

    while app.next_frame() {
        draw_world_simple(&mut app, &world);
        world = update_world(&world, rule_string);
    }
}

fn count_neighbors(x: usize, y: usize, world: &[Vec<bool>]) -> usize {
    let height = world.len();
    let width = world[0].len();
    let mut neighbors: usize = 0;

    for ny in y.saturating_sub(1)..=(y + 1).min(height - 1) {
        for nx in x.saturating_sub(1)..=(x + 1).min(width - 1) {
            if (nx, ny) != (x, y) && world[ny][nx] {
                neighbors += 1;
            }
        }
    }

    neighbors
}

fn update_world(world: &[Vec<bool>], rule_string: &str) -> Vec<Vec<bool>> {
    let height = world.len();
    let width = world[0].len();
    let mut new_world: Vec<Vec<bool>> = Vec::new();

    for y in 0..height {
        new_world.push(Vec::new());
        for x in 0..width {
            let neighbors = count_neighbors(x, y, &world[..]);
            let mut rule_string = rule_string.split("/");

            if world[y][x] {
                if rule_string.last().unwrap().contains(&neighbors.to_string()) {
                    new_world[y].push(true);
                } else {
                    new_world[y].push(false);
                }
            } else {
                if rule_string.next().unwrap().contains(&neighbors.to_string()) {
                    new_world[y].push(true);
                } else {
                    new_world[y].push(false);
                }
            }
        }
    }

    new_world
}

fn draw_world_terminal(world: &[Vec<bool>]) {
    let height = world.len();
    let width = world[0].len();

    println!();
    println!();
    println!();
    println!();
    println!();

    for y in 0..height {
        for x in 0..width {
            if world[y][x] {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn draw_world_simple(app: &mut simple::Window, world: &[Vec<bool>]) {
    let height = world.len();
    let width = world[0].len();

    app.clear();

    for y in 0..height {
        for x in 0..width {
            if world[y][x] {
                app.set_color(255, 255, 255, 255);
                app.draw_rect(simple::Rect::new((x * 2) as i32, (y * 2) as i32, 2, 2));
            }
        }
    }
}