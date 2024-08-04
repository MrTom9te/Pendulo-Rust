#[warn(dead_code)]
use speedy2d::{
    color::Color,
    font::{Font, TextLayout, TextOptions},
    window::WindowHandler,
    Graphics2D, Window,
};
use std::{f32, fmt::format, io::stdin};

use vector::Vector;

fn main() {
    let mut quantity = String::new();
    println!("Quantos pendulos?");
    stdin().read_line(&mut quantity).unwrap();
    let quantity: f32 = match quantity.trim().parse() {
        Ok(numero) => numero,
        Err(_) => {
            println!("Erro: quantidade setada para 1 ");
            1f32
        }
    };

    let window = Window::new_centered("Pendulum", (1000, 600)).unwrap();
    let my_window_handler = |x: f32| -> MyWindowHandler {
        let mut wh = MyWindowHandler { pendulos: vec![] };
        let counter = 950f32 / x;
        if x < 2f32 {
            wh.pendulos.push(Pendulum::new(500f32, 0f32, 200f32));
        } else {
            for i in 0..x as i32 {
                wh.pendulos.push(Pendulum::new(
                    counter + ((i as f32) * counter),
                    0f32,
                    200f32,
                ))
            }
        }

        wh
    };

    let win = my_window_handler;
    window.run_loop(win(quantity));
}
struct MyWindowHandler {
    pendulos: Vec<Pendulum>,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut Graphics2D,
    ) {
        graphics.clear_screen(Color::TRANSPARENT);
        let bytes = include_bytes!("assets/fonts/noto-sans/NotoSans-Regular.ttf");
        let font = Font::new(bytes).unwrap();

        for text in &self.pendulos {
            let data1 = text.position.x.to_string();
            let data2 = text.position.y.to_string();
            let text_position = format(format_args!("x:{}\ny{}", data1, data2));
            let block = font.layout_text(&text_position, 19f32, TextOptions::new());
            graphics.draw_text(
                (text.position.x - 30f32, text.position.y + 30f32),
                Color::from_rgb(205f32, 47f32, 210f32),
                &block,
            );
        }

        for i in &mut self.pendulos {
            i.draaw(graphics);
            i.update();
        }

        //self.pendulos[0].draaw(graphics);
        //self.pendulos[1].update();
        //self.pendulos[0].draaw(graphics);
        //self.pendulos[1].draaw(graphics);

        for i in &mut self.pendulos {}

        helper.request_redraw();
    }
}

#[derive(Debug)]
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
            r,
            m: 0.5,
            g: 0.5,
        }
    }

    fn update(&mut self) {
        self.angular_aceleration += -1f32 * self.angle.sin() * self.g / self.r;

        self.angular_velocity += self.angular_aceleration;

        self.angle += self.angular_aceleration;

        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        self.position.add(&self.origim);
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
