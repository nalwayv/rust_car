use sfml::{graphics::*, system::*, window::*};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::hash::Hash;

/// draw tile on screen
fn draw_tile(
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    tx: i32,
    ty: i32,
    texture: &Texture,
    window: &mut RenderWindow,
) {
    let new_x = (x * w) as f32;
    let new_y = (y * h) as f32;
    let new_tx = tx * w;
    let new_ty = ty * h;

    let mut sp = Sprite::with_texture(texture);
    sp.set_position(Vector2f::new(new_x, new_y));
    sp.set_origin((0.0, 0.0));

    sp.set_texture_rect(&IntRect::new(new_tx, new_ty, w, h));

    window.draw(&sp);
}

/// deg to rad
fn d2r(deg: f32) -> f32 {
    deg / 360.0 * (PI * 2.0)
}

/// rad to deg
fn r2d(rad: f32) -> f32 {
    (rad / (PI * 2.0)) * 360.0
}

// V2 HELPERS ---
#[allow(dead_code)]
fn v2_dot(a: Vector2f, b: Vector2f) -> f32 {
    let x = a.x * b.x;
    let y = a.y * b.y;
    x + y
}

#[allow(dead_code)]
fn v2_length(v2: Vector2f) -> f32 {
    v2_dot(v2, v2).sqrt()
}

#[allow(dead_code)]
fn v2_length_sq(v2: Vector2f) -> f32 {
    v2_dot(v2, v2)
}

#[allow(dead_code)]
fn v2_set_length(v2: Vector2f, new_length: f32) -> Vector2f {
    let ang = v2_angle(v2);
    v2_set_rotation(ang) * new_length
}

#[allow(dead_code)]
fn v2_set_angle(v2: Vector2f, ang: f32) -> Vector2f {
    let len = v2_length(v2);
    v2_set_rotation(ang) * len
}

#[allow(dead_code)]
fn v2_normalize(v2: Vector2f) -> Vector2f {
    v2 * (1. / v2_length(v2))
}

fn v2_angle_to_point(a: Vector2f, b: Vector2f) -> f32 {
    let dis = a - b;
    dis.y.atan2(dis.x)
}

fn v2_set_rotation(rad: f32) -> Vector2f {
    Vector2f::new(rad.cos(), rad.sin())
}

#[allow(dead_code)]
fn v2_angle(v: Vector2f) -> f32 {
    v.y.atan2(v.x)
}

/// rotate a vector by radians
fn v2_rotated(v: Vector2f, by: f32) -> Vector2f {
    // rotation matrix
    // [ cos - sin ]
    // [ sin + cos ]
    let new_x = (v.x * by.cos()) - (v.y * by.sin());
    let new_y = (v.x * by.sin()) + (v.y * by.cos());

    Vector2f::new(new_x, new_y)
}

// TILE LAYERS ---

struct TileLayer {
    rows: i32,
    cols: i32,
    tile_sheet_cols: i32,
    tile_sheets: [[i32; 70]; 2],
    tile_sheets_info: [[(bool, i32, i32, i32, i32); 70]; 2],
}

impl TileLayer {
    fn new() -> Self {
        let track = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 149, 185, 185, 185, 185, 185, 185, 131, 0, 0, 202, 94,
            183, 183, 183, 183, 76, 166, 0, 0, 307, 271, 0, 0, 0, 0, 202, 166, 0, 0, 202, 93, 185,
            185, 185, 185, 75, 166, 0, 0, 148, 183, 183, 183, 183, 183, 183, 130, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];

        let grass = [
            102, 281, 281, 281, 281, 281, 281, 281, 281, 84, 30, 70, 70, 70, 70, 70, 70, 70, 70,
            66, 30, 70, 70, 70, 70, 70, 70, 70, 70, 66, 30, 70, 70, 70, 70, 70, 70, 70, 70, 66, 30,
            70, 70, 70, 70, 70, 70, 70, 70, 66, 30, 70, 70, 70, 70, 70, 70, 70, 70, 66, 12, 209,
            209, 209, 209, 209, 209, 209, 209, 317,
        ];

        Self {
            rows: 7,
            cols: 10,
            tile_sheet_cols: 18,
            tile_sheets: [grass, track],
            tile_sheets_info: [[(false, 0, 0, 0, 0); 70]; 2],
        }
    }

    fn get_info(&self) -> &[[(bool, i32, i32, i32, i32); 70]; 2] {
        &self.tile_sheets_info
    }

    fn set_up(&mut self) {
        for i in 0..2 {
            //
            for x in 0..self.cols {
                for y in 0..self.rows {
                    let coord = x + self.cols * y;

                    if let Some(num) = self.tile_sheets[i].get(coord as usize) {
                        if *num <= 0 {
                            continue;
                        }

                        let tx = num % self.tile_sheet_cols;
                        let ty = num / self.tile_sheet_cols;

                        self.tile_sheets_info[i][coord as usize] = (true, x, y, tx, ty);
                    }
                }
            }
            //
        }
    }
}

// TEXTURE MANAGER ---
pub struct TextureManager<ID: Hash + Eq> {
    // sfbox -> pointer to an SFML-allocated object
    texture_map: HashMap<ID, SfBox<Texture>>,
}

impl<ID> TextureManager<ID>
where
    ID: Hash + Eq,
{
    /// add texture
    pub fn load(&mut self, id: ID, file_path: &str) {
        let new_texture = Texture::from_file(file_path).unwrap();
        self.texture_map.insert(id, new_texture);
    }

    /// get texture by its id
    pub fn get(&self, key: ID) -> &Texture {
        &self.texture_map.get(&key).unwrap()
    }
}

// set default values
impl<ID> Default for TextureManager<ID>
where
    ID: Hash + Eq,
{
    fn default() -> Self {
        Self {
            texture_map: HashMap::default(),
        }
    }
}


// CAR ---
struct Car {
    position: Vector2f,
    velocity: Vector2f,
    acceleration: Vector2f,
    force: Vector2f,

    back_wheel: Vector2f,
    front_wheel: Vector2f,

    axel: f32,
    speed: f32,
    friction: f32,
    drag: f32,
    // breaking: f32,
    car_angle: f32,
    steer_angle: f32,

    is_moving: bool,
    is_reversing: bool,
    is_turning_left: bool,
    is_turning_right: bool,
}

impl Car {
    fn new(pos: Vector2f) -> Self {
        Self {
            position: pos,
            velocity: Vector2f::new(0.0, 0.0),
            acceleration: Vector2f::new(0.0, 0.0),
            force: Vector2f::new(0.0, 0.0),

            back_wheel: Vector2f::new(0.0, 0.0),
            front_wheel: Vector2f::new(0.0, 0.0),

            axel: 45.0, // distance between front and back wheel
            speed: 750.0,
            friction: -0.99,
            drag: -0.0055,

            car_angle: 0.0,
            steer_angle: 75.0,

            is_moving: false,
            is_reversing: false,
            is_turning_left: false,
            is_turning_right: false,
        }
    }

    fn get_angle(&self) -> f32 {
        self.car_angle
    }

    fn get_position(&self) -> Vector2f {
        self.position
    }

    fn steering(&mut self, delta: f32) {
        let mut front = self.position + self.axel / 2.0 * v2_set_rotation(self.car_angle);
        let mut back = self.position - self.axel / 2.0 * v2_set_rotation(self.car_angle);

        back += self.velocity * delta;
        front += v2_rotated(self.velocity, self.steer_angle) * delta;

        self.car_angle = v2_angle_to_point(front, back);

        self.front_wheel = front;
        self.back_wheel = back;
    }

    fn screen_wrap(&mut self, width: f32, height: f32, buffer: f32) {
        let screen_edge = 0.;

        if self.position.x > width + buffer {
            self.position.x = screen_edge - buffer;
        }
        if self.position.x < screen_edge - buffer {
            self.position.x = width + buffer;
        }
        if self.position.y > height + buffer {
            self.position.y = screen_edge - buffer;
        }
        if self.position.y < screen_edge - buffer {
            self.position.y = height + buffer;
        }
    }

    fn update_forces(&mut self, _delta: f32) {
        let speed = v2_length(self.velocity);

        if speed < 20.0 {
            self.velocity = Vector2f::new(0.0, 0.0);
        }

        let mut f_force =  self.velocity * self.friction;
        let d_force = self.velocity * speed * self.drag;

        if speed < 100.0 {
            f_force *= 3.0;
        }

        // total 
        self.force = f_force + d_force;
        let total = self.force / 1.0;

        self.acceleration += total;
    }

    fn update(&mut self, dt: f32) {
        let mut t = 0.0;

        if self.is_turning_left {
            t -= 1.0;
        }

        if self.is_turning_right {
            t += 1.0;
        }

        self.steer_angle = t * d2r(15.0);

        if self.is_moving {
            self.acceleration = v2_set_rotation(self.car_angle) * self.speed;
        } else {
            self.acceleration = Vector2f::new(0.0, 0.0);
        }

        if self.is_reversing {
            self.acceleration = Vector2f::new(1.0, 0.0) + v2_set_rotation(self.car_angle) * -600.0;
        }

        self.steering(dt);

        self.update_forces(dt);

        self.screen_wrap(1280.0, 896.0, 25.0);

        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
        self.acceleration *= 0.0;
    }
}

// MAIN ---

fn run(width: u32, height: u32) {
    let mut window = RenderWindow::new((width, height), "CAR", Style::CLOSE, &Default::default());
    window.set_mouse_cursor_visible(true);
    window.set_framerate_limit(30);
    let mut is_paused = false;
    let mut clock = Clock::start();

    let mut tm = TextureManager::default();
    tm.load("car", "assets/img/car.png");
    tm.load("sheet", "assets/img/spritesheet_tiles.png");

    let mut car = Car::new(Vector2f::new(100.0, 100.0));

    let mut car_texture = Sprite::with_texture(tm.get("car"));
    let mut shadow = Sprite::with_texture(tm.get("car"));
    car_texture.set_texture_rect(&IntRect::new(0, 0, 33, 18));
    car_texture.set_origin((33.0 / 2.0, 18.0 / 2.0));
    car_texture.set_position(car.get_position());

    shadow.set_texture_rect(&IntRect::new(0, 0, 33, 18));
    shadow.set_color(Color::rgba(21, 21, 21, 50));
    shadow.set_origin((33.0 / 2.0, 18.0 / 2.0));
    shadow.set_position(car.get_position());

    let mut tl = TileLayer::new();
    tl.set_up();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => window.close(),
                    Key::P => is_paused = !is_paused,
                    // car
                    Key::W => {
                        car.is_moving = true;
                    }
                    Key::A => {
                        car.is_turning_left = true;
                    }
                    Key::D => {
                        car.is_turning_right = true;
                    }
                    Key::S => {
                        car.is_reversing = true;
                    }
                    _ => {}
                },
                Event::KeyReleased { code, .. } => match code {
                    Key::W => {
                        car.is_moving = false;
                    }
                    Key::A => {
                        car.is_turning_left = false;
                    }
                    Key::D => {
                        car.is_turning_right = false;
                    }
                    Key::S => {
                        car.is_reversing = false;
                    }
                    _ => {}
                },
       
                _ => {}
            }
        }

        if !is_paused {
            let delta = clock.restart().as_seconds();

            car.update(delta);

            car_texture.set_position(car.get_position());
            car_texture.set_rotation(r2d(car.get_angle()));

            shadow.set_position(car.get_position() + Vector2f::new(3.0, 3.0));
            shadow.set_rotation(r2d(car.get_angle()));

            // draw ---
            window.clear(Color::rgb(21, 21, 21));

            for x in tl.get_info().iter() {
                for y in x.iter() {
                    if y.0 {
                        let xpos = y.1;
                        let ypos = y.2;
                        let txpos = y.3;
                        let typos = y.4;

                        draw_tile(
                            xpos,
                            ypos,
                            128,
                            128,
                            txpos,
                            typos,
                            tm.get("sheet"),
                            &mut window,
                        );
                    }
                }
            }

            window.draw(&shadow);
            window.draw(&car_texture);

            window.display();
        } else {
            clock.restart();
        }
    }
}

fn main() {
    run(1280, 896);
}
