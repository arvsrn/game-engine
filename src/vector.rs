use std::fmt::Debug;

#[derive(Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn add(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn mul(&self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }

    pub fn len(&self) -> f32 {
        f32::sqrt((self.x.powf(2.0) + self.y.powf(2.0)) as f32)
    }

    /// Basis is a tuple of (x, y)
    pub fn transform(&self, basis: (Vec2, Vec2)) -> Vec2 {
        Vec2 {
            x: self.x * basis.0.x + self.y * basis.1.x,
            y: self.x * basis.0.y + self.y * basis.1.y,
        }
    }

    pub fn euclid_distance(&self, other: Vec2) -> f32 {
        f32::sqrt(((other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)) as f32)
    }

    pub fn l1_distance(&self, other: Vec2) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

#[derive(Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn add(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn len(&self) -> f32 {
        f32::sqrt((self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)) as f32)
    }

    pub fn transform(&self, basis: (f32, f32, f32, f32, f32, f32, f32, f32, f32)) -> Vec3 {
        Vec3 {
            x: self.x * basis.0 + self.y * basis.1 + self.z * basis.2,
            y: self.x * basis.3 + self.y * basis.4 + self.z * basis.5,
            z: self.x * basis.6 + self.y * basis.7 + self.z * basis.8,
        }
    }

    /// returns the x and y value in Vec2 form
    pub fn as_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
