use minifb::Key;
use render::Shape;
use vector::Vec2;

mod color;
mod render;
mod vector;

fn main() {
    let mut window = render::Window::new("Graphics");
    window.shapes.push(Shape::Triangle {
        a: Vec2::new(158.0, 100.0),
        b: Vec2::new(100.0, 110.0),
        c: Vec2::new(206.0, 107.0),
        shader: |_, _, _| (155, 155, 155),
    });
    window.shapes.push(Shape::Triangle {
        a: Vec2::new(149.0, 123.0),
        b: Vec2::new(100.0, 110.0),
        c: Vec2::new(206.0, 107.0),
        shader: |_, _, _| (155, 155, 155),
    });
    window.shapes.push(Shape::Triangle {
        a: Vec2::new(149.0, 123.0),
        b: Vec2::new(100.0, 110.0),
        c: Vec2::new(157.0, 201.0),
        shader: |_, _, _| (200, 200, 200),
    });
    window.shapes.push(Shape::Triangle {
        a: Vec2::new(111.0, 177.0),
        b: Vec2::new(100.0, 110.0),
        c: Vec2::new(157.0, 201.0),
        shader: |_, _, _| (200, 200, 200),
    });
    window.shapes.push(Shape::Triangle {
        a: Vec2::new(149.0, 123.0),
        b: Vec2::new(157.0, 201.0),
        c: Vec2::new(206.0, 107.0),
        shader: |_, _, _| (255, 255, 255),
    });
    window.shapes.push(Shape::Triangle {
        a: Vec2::new(206.0, 172.0),
        b: Vec2::new(157.0, 201.0),
        c: Vec2::new(206.0, 107.0),
        shader: |_, _, _| (255, 255, 255),
    });

    while window.window.is_open() {
        //window.draw_polygons();
        if window.window.is_key_down(Key::Escape) {
            break;
        }
        
        window.process();
        window.projection();
        window.update();
    }
}
