use std::ops::Rem;
use std::time::Instant;

use minifb::{Key, Window as Window_, WindowOptions};

use crate::{
    color::{mix, RGB},
    vector::{Vec2, Vec3},
};

const WIDTH: usize = 540;
const HEIGHT: usize = 360;
const BACKGROUND: u32 = 0x101010;

fn triangle_area(p1: Vec2, p2: Vec2, p3: Vec2) -> u32 {
    ((p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y)) / 2.0).abs() as u32
}

#[inline(always)]
fn min(a: f32, b: f32) -> f32 {
    return if a > b { b } else { a };
}

#[inline(always)]
fn max(a: f32, b: f32) -> f32 {
    return if a < b { b } else { a };
}

#[derive(Clone)]
pub enum Shape {
    Triangle {
        a: Vec2,
        b: Vec2,
        c: Vec2,
        shader: fn(u32, u32, u32) -> RGB,
    },
}

pub struct Polygon3D {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    color: fn(u32, u32, u32) -> RGB,
}

pub struct Window {
    pub window: Window_,
    pub buffer: Vec<u32>,
    pub shapes: Vec<Shape>,
    pub camera_translation: Vec3,
    pub camera_rotation: Vec3,
    last_mouse_position: Vec2,
}

impl Window {
    pub fn new(name: &str) -> Window {
        let buffer: Vec<u32> = vec![BACKGROUND; WIDTH * HEIGHT];
        let mut window = Window_::new(name, WIDTH, HEIGHT, WindowOptions::default()).unwrap();

        // window.set_cursor_visibility(false);

        Window {
            buffer,
            window,
            shapes: vec![],
            camera_rotation: Vec3::new(0.0, 0.0, 0.0),
            camera_translation: Vec3::new(0.0, 0.0, 0.0),
            last_mouse_position: Vec2::new(0.0, 0.0),
        }
    }

    fn draw_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, shader: fn(u32, u32, u32) -> RGB) {
        // we figure out the bounding box of the triangle, then iterate through each pixel in that bounding box and check if the pixel is in triangle
        let min_x = min(a.x, min(b.x, c.x));
        let min_y = min(a.y, min(b.y, c.y));
        let max_x = max(a.x, max(b.x, c.x));
        let max_y = max(a.y, max(b.y, c.y));

        let width = max_x - min_x;
        let height = max_y - min_y;
        let top = min_y;
        let left = min_x;

        if width as u32 == 0 || height as u32 == 0 {
            println!("Width or height is zero, failed to draw triangle. Dimensions: {} by {} pixels", width, height);
            return 
        }

        if left + width < 0.0 && top + height < 0.0 {
            println!("Triangle is completely out of frame, failed to draw triangle. Position ({}, {})", left, top);
            return;
        }

        let bounding_box_area = width * height;
        let abc = triangle_area(a.clone(), b.clone(), c.clone());
        // let light_position = Vec2::new(540.0, 360.0);

        for i in 1..(bounding_box_area as u32) {
            let p = Vec2::new(i.rem(width as u32) as f32, (i / width as u32) as f32);

            let p_global = p.add(Vec2::new(left, top));

            let pbc = triangle_area(p_global.clone(), b.clone(), c.clone());
            let apc = triangle_area(a.clone(), p_global.clone(), c.clone());
            let abp = triangle_area(a.clone(), b.clone(), p_global.clone());

            if (abc > pbc + apc + abp - 5) && (abc < pbc + apc + abp + 5)
                && (p_global.x >= 0.0) && (p_global.x < WIDTH as f32) && (p_global.y >= 0.0) && (p_global.y < HEIGHT as f32) 
            {
                let base_color = (shader)(p.x as u32, p.y as u32, 0);
                // let distance = 1.0 - (p_global.euclid_distance(light_position.clone()) as f32 / 649.0);

                // multiply each base_color.n by distance for (wonky) shading
                let final_color = (base_color.0 as f32).round() as u32 * 0x10000
                    + (base_color.1 as f32).round() as u32 * 0x100
                    + (base_color.2 as f32).round() as u32;

                let index = (WIDTH as u32 * max(0.0, p_global.y) as u32 + max(0.0, p_global.x) as u32) as usize;
                self.buffer[index] = final_color;
            } /* else if abc > pbc + apc + abp - 100 && abc < pbc + apc + abp
                && (p_global.x >= 0.0) && (p_global.x < WIDTH as f32) && (p_global.y >= 0.0) && (p_global.y < HEIGHT as f32) 
            {
                // super wonky anti-aliasing
                let background = self.buffer[(WIDTH as u32 * (top + p.y) as u32 + left as u32 + p.x as u32) as usize];
                let background_rgb = (
                    ((background >> (8*0)) & 0xff) as u8,
                    ((background >> (8*1)) & 0xff) as u8,
                    ((background >> (8*2)) & 0xff) as u8
                );

                let base_color = (shader)(p.x as u32, p.y as u32, 0);
                let blended = mix(base_color, background_rgb, 1.0 - ((pbc + apc + abp) - abc) as f32/100.0);

                self.buffer[(WIDTH as u32 * (top + p.y) as u32 + left as u32 + p.x as u32) as usize] =
                    blended.0 as u32 * 0x10000 + blended.1 as u32 * 0x100 + blended.2 as u32;
            } */
        }
    }

    pub fn draw_polygons(&mut self) {
        let instant: Instant = Instant::now();

        for shape in self.shapes.clone() {
            match shape {
                Shape::Triangle { a, b, c, shader } => self.draw_triangle(a, b, c, shader),
            }
        }

        println!("Drawn polygons in {:?}", instant.elapsed());
    }

    pub fn projection(&mut self) {
        let instant: Instant = Instant::now();

        let polygons = vec![
            Polygon3D {
                a: Vec3::new(0.0, 0.0, 1.0),
                b: Vec3::new(0.0, 1.0, 1.0),
                c: Vec3::new(1.0, 0.0, 1.0),
                color: |_,_,_| (155, 155, 155),
            },
            Polygon3D {
                a: Vec3::new(1.0, 1.0, 1.0),
                b: Vec3::new(0.0, 1.0, 1.0),
                c: Vec3::new(1.0, 0.0, 1.0),
                color: |_,_,_| (155, 155, 155),
            },
            Polygon3D {
                a: Vec3::new(1.0, 1.0, 1.0),
                b: Vec3::new(0.0, 1.0, 1.0),
                c: Vec3::new(0.0, 1.0, 2.0),
                color: |_,_,_| (200, 200, 200),
            },
            Polygon3D {
                a: Vec3::new(1.0, 1.0, 1.0),
                b: Vec3::new(1.0, 1.0, 2.0),
                c: Vec3::new(0.0, 1.0, 2.0),
                color: |_,_,_| (200, 200, 200),
            },
        ];

        for polygon in polygons {
            let mut i = 0;
            let mut draw = true;

            // points of the polygon in screen space
            let mut screen_points = [
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, 0.0),
            ];

            // points of the polygon in game space
            let points = [
                polygon.a.sub(self.camera_translation.clone()),
                polygon.b.sub(self.camera_translation.clone()),
                polygon.c.sub(self.camera_translation.clone()),
            ];

            for mut point in points {
                // rotation on the x axis
                point = point.transform((
                    1.0, 0.0, 0.0,
                    0.0, self.camera_rotation.x.to_radians().cos(), -self.camera_rotation.x.to_radians().sin(),
                    0.0, self.camera_rotation.x.to_radians().sin(), self.camera_rotation.x.to_radians().cos(),
                ));
                
                // rotation on the y axis
                point = point.transform((
                    self.camera_rotation.y.to_radians().cos(), 0.0, self.camera_rotation.y.to_radians().sin(),
                    0.0, 1.0, 0.0,
                    -self.camera_rotation.x.to_radians().sin(), 0.0, self.camera_rotation.y.to_radians().cos(),
                ));

                if point.z < 0.0 {
                    draw = false;
                    break;
                }

                let screen_space_point = Vec3::new(
                    (1.0 * point.x) / point.z,
                    (1.0 * point.y) / point.z,
                    ((101.0 * point.z) - 100.0) / point.z,
                );

                screen_points[i] = Vec2::new(
                    270.0 - (screen_space_point.x * 100.0),
                    180.0 - (screen_space_point.y * 100.0),
                );
                i += 1;
            }

            if draw {
                self.draw_triangle(
                    screen_points[0].clone(),
                    screen_points[1].clone(),
                    screen_points[2].clone(),
                    polygon.color,
                );
            }   
        }

        println!("Drawn projection in {:?}", instant.elapsed());
    }

    pub fn process(&mut self) {
        if self.window.is_key_down(Key::A) {
            self.camera_translation.x += 0.1;
        }
        if self.window.is_key_down(Key::D) {
            self.camera_translation.x -= 0.1;
        }
        if self.window.is_key_down(Key::W) {
            self.camera_translation.z += 0.1;
        }
        if self.window.is_key_down(Key::S) {
            self.camera_translation.z -= 0.1;
        }
        if self.window.is_key_down(Key::Q) {
            self.camera_translation.y += 0.1;
        }
        if self.window.is_key_down(Key::E) {
            self.camera_translation.y -= 0.1;
        }
        if self.window.is_key_pressed(Key::Z, minifb::KeyRepeat::Yes) {
            self.camera_rotation.x -= 5.0;
            println!("{}", self.camera_rotation.x);
        }
        if self.window.is_key_pressed(Key::C, minifb::KeyRepeat::Yes) {
            self.camera_rotation.x += 5.0;
            println!("{}", self.camera_rotation.x);
        }

        match self.window.get_mouse_pos(minifb::MouseMode::Discard) {
            Some((x, y)) => {
                // let dx = x - self.last_mouse_position.x;
                // self.camera_rotation.y += dx * 0.1;
                // let dy = y - self.last_mouse_position.y;
                // self.camera_rotation.x += dy * 0.1;
                // 
                // self.camera_rotation.x = max(-90.0, min(self.camera_rotation.x, 90.0));
                // 
                // self.last_mouse_position = Vec2::new(x, y);
                // self.window.set_mouse_pos(180, 270);
            }
            None => {}
        }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();

        self.buffer = vec![BACKGROUND; WIDTH * HEIGHT];
    }
}
