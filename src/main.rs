use game_of_life::lib;

fn main() {
    let rule_string = "B3/S23";

    let mut world: Vec<Vec<bool>> = Vec::new();
    let width: usize = 512;
    let height: usize = 512;

    let mut pixels_per_cell = 1;
    let mut xoffset = 0;
    let mut yoffset = 0;
    let mut running = false;
    let mut can_toggle = true;

    let mut app = simple::Window::new("Rusty Life", 512, 512);

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
    //     std::thread::sleep(std::time::Duration::from_millis(100));
    // }

    while app.next_frame() {
        lib::draw_world_simple(&mut app, &mut world, &mut pixels_per_cell, &mut xoffset, &mut yoffset);

        if app.is_key_down(simple::Key::Space) && can_toggle {
            running = !running;
            can_toggle = false;
        } else if !app.is_key_down(simple::Key::Space) && !can_toggle {
            can_toggle = true;
        }

        if running {
            world = lib::update_world(&world, rule_string);
        }
        // std::thread::sleep(std::time::Duration::from_millis(250));
    }
}