use piston::input::UpdateArgs;
use cgmath::*;

#[derive(Debug)]
pub struct Player {
    pub rotation: Rad<f64>,
    pub position: Vector2<f64>,
}

impl Player {
    pub fn update(&mut self, args: &UpdateArgs, update_command: &PlayerUpdate) {
        if update_command.move_east {
            self.position.x += 200.0 * args.dt;
        }

        if update_command.move_west {
            self.position.x -= 200.0 * args.dt;
        }

        if update_command.move_north {
            self.position.y -= 200.0 * args.dt;
        }

        if update_command.move_south {
            self.position.y += 200.0 * args.dt;
        }
    }
}

#[derive(Debug)]
pub struct PlayerUpdate {
    pub rotate_clockwise: bool,
    pub rotate_counter_clockwise: bool,
    pub move_north: bool,
    pub move_south: bool,
    pub move_east: bool,
    pub move_west: bool,
}

impl PlayerUpdate {
    pub fn new() -> Self {
        PlayerUpdate {
            rotate_clockwise: false,
            rotate_counter_clockwise: false,
            move_north: false,
            move_south: false,
            move_east: false,
            move_west: false,
        }
    }
}
