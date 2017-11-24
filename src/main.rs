extern crate cgmath;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use cgmath::*;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod player;
use player::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player: Player,
    player_update_command: PlayerUpdate,
    current_mouse_pos: Vector2<f64>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let Player { rotation, position, ..} = self.player;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            let transform = c.transform.trans(position.x, position.y)
                                       .rot_rad(rotation.0)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let &mut App { ref mut player, ref player_update_command, ref current_mouse_pos, .. } = self;
        player.update(args, player_update_command);
        player.point_to(args, current_mouse_pos);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let width = 800;
    let height = 800;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [width, height]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let center = Vector2::new(width as f64/2.0, height as f64/2.0);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player_update_command: PlayerUpdate::new(),
        current_mouse_pos: center,
        player: Player {
            rotation: Rad(0.0),
            position: center,
            velocity: Vector2::new(0.0, 0.0),
        },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(p) = e.press_args() {
            match p {
                Button::Keyboard(Key::W) => app.player_update_command.move_north = true,
                Button::Keyboard(Key::A) => app.player_update_command.move_west = true,
                Button::Keyboard(Key::S) => app.player_update_command.move_south = true,
                Button::Keyboard(Key::D) => app.player_update_command.move_east = true,
                _ => {},
            };
        }

        if let Some(p) = e.release_args() {
            match p {
                Button::Keyboard(Key::W) => app.player_update_command.move_north = false,
                Button::Keyboard(Key::A) => app.player_update_command.move_west = false,
                Button::Keyboard(Key::S) => app.player_update_command.move_south = false,
                Button::Keyboard(Key::D) => app.player_update_command.move_east = false,
                _ => {},
            };
        }

        if let Some(xy) = e.mouse_cursor_args() {
            app.current_mouse_pos = Vector2::from(xy);
        }
    }
}
