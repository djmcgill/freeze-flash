use piston::input::UpdateArgs;
use cgmath::*;

#[derive(Debug)]
pub struct Player {
    pub rotation: Rad<f64>,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
}

fn bound(min: f64, max: f64, x: f64) -> f64 {
    if x <= min { return min; }
    if x >= max { return max; }
    x
}

const SPEED_LIMIT: f64 = 200.0;
const MOTION_CUTOFF: f64 = 0.1;
const ACCELERATION: f64 = 50.0;
const FRICTION: f64 = 7.0;

impl Player {
    pub fn update(&mut self, args: &UpdateArgs, update_command: &PlayerUpdate) {
        match (update_command.move_east, update_command.move_west) {
            (true, false) => self.velocity.x += ACCELERATION, // speed up left
            (false, true) => self.velocity.x -= ACCELERATION, // speed up right
            _ => self.velocity.x =
                self.velocity.x.signum() * bound(0.0, SPEED_LIMIT, self.velocity.x.abs() - FRICTION), // slow down
        };

        match (update_command.move_north, update_command.move_south) {
            (true, false) => self.velocity.y -= ACCELERATION, // speed up up
            (false, true) => self.velocity.y += ACCELERATION, // speed up down
            _ => self.velocity.y =
                self.velocity.y.signum() * bound(0.0, SPEED_LIMIT, self.velocity.y.abs() - FRICTION), // slow down
        };
        
        self.velocity.x = bound(-SPEED_LIMIT, SPEED_LIMIT, self.velocity.x);
        if self.velocity.x.abs() < MOTION_CUTOFF { self.velocity.x = 0.0 };
        self.velocity.y = bound(-SPEED_LIMIT, SPEED_LIMIT, self.velocity.y);
        if self.velocity.y.abs() < MOTION_CUTOFF { self.velocity.y = 0.0 };
        
        self.position += self.velocity * args.dt;
    }

    pub fn point_to(&mut self, _args: &UpdateArgs, point_to: &Vector2<f64>) {
        let p_to_m = point_to - self.position;
        let angle_from_vert = Rad::atan2(p_to_m.y, p_to_m.x);
        self.rotation = angle_from_vert;
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
