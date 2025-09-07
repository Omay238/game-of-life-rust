pub mod lib {

    pub fn count_neighbors(x: usize, y: usize, world: &[Vec<bool>]) -> usize {
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

    pub fn update_world(world: &[Vec<bool>], rule_string: &str) -> Vec<Vec<bool>> {
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


    // pub fn draw_world_terminal(world: &[Vec<bool>]) {
    //     let height = world.len();
    //     let width = world[0].len();
    //
    //     println!();
    //     println!();
    //     println!();
    //     println!();
    //     println!();
    //
    //     for y in 0..height {
    //         for x in 0..width {
    //             if world[y][x] {
    //                 print!("█");
    //             } else {
    //                 print!(" ");
    //             }
    //         }
    //         println!();
    //     }
    // }

    pub fn draw_world_simple(app: &mut simple::Window, world: &mut Vec<Vec<bool>>, pixels_per_cell: &mut i32, xoffset: &mut i32, yoffset: &mut i32) {
        let height = world.len();
        let width = world[0].len();


        let mouse_pos = app.mouse_position();
        let mouse_pos = ((mouse_pos.0 / *pixels_per_cell - *xoffset) as usize, (mouse_pos.1 / *pixels_per_cell - *yoffset) as usize);
        if mouse_pos.0 < width && mouse_pos.1 < height {
            if app.is_mouse_button_down(simple::MouseButton::Left) {
                world[mouse_pos.1][mouse_pos.0] = true;
            }
            if app.is_mouse_button_down(simple::MouseButton::Right) {
                world[mouse_pos.1][mouse_pos.0] = false;
            }
        }

        if app.is_key_down(simple::Key::W) {
            *yoffset += 1;
        }
        if app.is_key_down(simple::Key::S) {
            *yoffset -= 1;
        }
        if app.is_key_down(simple::Key::A) {
            *xoffset += 1;
        }
        if app.is_key_down(simple::Key::D) {
            *xoffset -= 1;
        }

        if app.is_key_down(simple::Key::Q) {
            *pixels_per_cell -= 1;
        }
        if app.is_key_down(simple::Key::E) {
            *pixels_per_cell += 1;
        }

        *pixels_per_cell = *pixels_per_cell.min(&mut 128).max(&mut 1);

        app.clear();

        for y in 0..height {
            for x in 0..width {
                if world[y][x] {
                    app.set_color(255, 255, 255, 255);
                    app.fill_rect(simple::Rect::new(
                        (x as i32 + *xoffset) * *pixels_per_cell,
                        (y as i32 + *yoffset) * *pixels_per_cell,
                        *pixels_per_cell as u32,
                        *pixels_per_cell as u32
                    ));
                    // app.draw_point(simple::Point::new(x as i32, y as i32));
                }
            }
        }
    }
}