use speedy2d::{color::Color, window::WindowHandler, Graphics2D, Window};
use std::f32;

use vector::Vector;

fn main() {
    let window = Window::new_centered("Pendulum", (1000, 600)).unwrap();
    let my_window_handler = MyWindowHandler {
        p: (
            Pendulum::new(500f32, 0f32, 100f32),
            Pendulum::new(300f32, 0f32, 100f32),
        ),
    };
    let win = my_window_handler;
    window.run_loop(win);
}
struct MyWindowHandler {
    p: (Pendulum, Pendulum),
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut Graphics2D,
    ) {
        graphics.clear_screen(Color::from_rgb(0.8, 10.0, 0.0));
        self.p.0.update();
        self.p.1.update();
        self.p.0.draaw(graphics);
        self.p.1.draaw(graphics);

        helper.request_redraw();
    }
}

struct Pendulum {
    origim: vector::Vector,
    position: vector::Vector,

    angle: f32,

    angular_velocity: f32,
    angular_aceleration: f32,

    r: f32,
    m: f32,
    g: f32,
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            origim: Vector::new(x, y),
            position: Vector::new(x, y),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_aceleration: 0.0,
            r: r,
            m: 1.0,
            g: 0.5,
        }
    }
    fn update(&mut self) {
        self.angular_aceleration += -2f32 * self.angle.sin() * self.g / self.r;

        self.angular_velocity += self.angular_aceleration;

        self.angle += self.angular_aceleration;

        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        self.position.add(&self.origim);

        print!(
            "{:?}\r{:?}\r",
            dbg!(&self.position),
            dbg!(self.angular_aceleration)
        );
    }

    fn draaw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origim.x, self.origim.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::BLUE);
    }
}

mod vector {
    #[derive(Debug)]
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }
    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y }
        }
        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}
