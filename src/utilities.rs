#[derive(Copy, Clone)]
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub enum Components{
    X, Y, Z
}

impl Vector3{

    pub fn srgb(&self) -> Self{
        self.pow(2.2f32)
    }

    pub fn from_int(v: u32) -> Self{
        let r = (v >> 16 & 0xff) as f32 / 255f32;
        let g = (v >> 8 & 0xff) as f32 / 255f32;
        let b = (v >> 0 & 0xff) as f32 / 255f32;
        Self::new(r, g, b).restrict_values()
    }

    pub fn to_linear(&self) -> Self{
        /*Matrix3::new(
        Vector3::new(0.4124564, 0.2126729, 0.0193339),
        Vector3::new(0.3575761, 0.7151522, 0.1191920),
        Vector3::new(0.1804375, 0.0721750, 0.9503041)
        ).multiply(*self)*/
        Self::new(
            self.x.powf(2.2),
            self.y.powf(2.2),
            self.z.powf(2.2),
        )
    }


    pub fn largest_component(&self) -> Components{
        if self.x.abs() > self.y.abs() {
            if self.x.abs() > self.z.abs() {
                return Components::X;
            }else {
                return Components::Z;
            }
        }else{
            if self.y.abs() > self.z.abs(){
                return Components::Y;
            }else{
                return Components::Z;
            }
        }
    }

    pub fn largest_component_value(&self) -> f32{
        match self.largest_component() {
            Components::X => self.x,
            Components::Y => self.y,
            Components::Z => self.z
        }
    }

    pub fn only_largest_component(&self) -> Self{
        match self.largest_component(){
            Components::X => Self::new(self.x, 0f32, 0f32),
            Components::Y => Self::new(0f32, self.y, 0f32),
            Components::Z => Self::new(0f32, 0f32, self.z)
        }
    }

    pub fn abs(&self) -> Self{
        Self::new(
            self.x.abs(),
            self.y.abs(),
            self.z.abs()
        )
    }

    pub fn zero() -> Self{
        Self::new(0f32, 0f32, 0f32)
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Self{
            x, y, z
        }
    }

    pub fn from_single(a: f32) -> Self{
        Self::new(a, a, a)
    }

    fn restrict_value(v: f32) -> f32{
        match v{
            v if v < 0f32 => 0f32,
            v  if v > 1f32 => 1f32,
            _ => v
        }
    }

    fn restrict_values(&self) -> Self{
        Self::new(
            Self::restrict_value(self.x),
            Self::restrict_value(self.y),
            Self::restrict_value(self.z)
        )
    }

    pub fn to_color_array(&self) -> [u8;3]{
        let x = Vector3::restrict_value(self.x);
        let y = Vector3::restrict_value(self.y);
        let z = Vector3::restrict_value(self.z);
        [(x*255f32) as u8,
         (y*255f32) as u8,
         (z*255f32) as u8]
    }

    pub fn subtract(&self, a: Self) -> Self{
        Self{
            x: self.x - a.x,
            y: self.y - a.y,
            z: self.z - a.z,
        }
    }

    pub fn add(&self, a: Self) -> Self{
        Self{
            x: self.x + a.x,
            y: self.y + a.y,
            z: self.z + a.z,
        }
    }

    pub fn length_squared(&self) -> f32{
        self.dot(*self)
    }

    pub fn length(&self) -> f32{
        self.length_squared().sqrt()
    }

    pub fn multiply(&self, scalar: f32) -> Self{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn normalized(&self) -> Self{
        let length = self.length();
        self.multiply(1f32/length)
    }

    pub fn dot(&self, a: Self) -> f32{
        self.x * a.x + self.y * a.y + self.z * a.z
    }

    pub fn reflect(&self, normal: Self) -> Self{
        let a = self.dot(normal);
        let p = self.multiply(1f32/a);
        normal.multiply(2f32).subtract(p).normalized()
    }
    pub fn comp_multiply(&self, a: Self) -> Self{
        Self::new(
            self.x * a.x,
            self.y * a.y,
            self.z * a.z,
        )
    }

    pub fn add_scalar(&self, a: f32) -> Self{
        Self::new(
            self.x + a,
            self.y + a,
            self.z + a,
        )
    }

    pub fn pow(&self, a: f32) -> Self{
        Self::new(
            self.x.powf(a),
            self.y.powf(a),
            self.z.powf(a),
        )
    }

    pub fn cross(&self, a: Self) -> Self{
        let x = self.y * a.z - self.z * a.y;
        let y = self.z * a.x - self.x * a.z;
        let z = self.x * a.y - self.y * a.x;
        Self::new(x, y, z)
    }

    pub fn max(&self, a: Self) -> Self{
        Self::new(
            self.x.max(a.x),
            self.y.max(a.y),
            self.z.max(a.z),
        )
    }

    pub fn min(&self, a: Self) -> Self{
        Self::new(
            self.x.min(a.x),
            self.y.min(a.y),
            self.z.min(a.z),
        )
    }
    //TODO: Verify accuracy
    pub fn refract(&self, n: Self, eta: f32) -> Self{
        let dotni = self.dot(n);
        let k = 1f32 - eta * eta * (1f32 - dotni * dotni);
        if k < 0f32{
            return Vector3::zero();
        }else{
            return self.multiply(eta).subtract(n.multiply(k.sqrt() + eta * self.dot(n)));
        }
    }
}


pub struct Matrix3{
    i: Vector3,
    j: Vector3,
    k: Vector3
}

impl Matrix3{

    pub fn new(i: Vector3, j: Vector3, k: Vector3) -> Self{
        Self{i, j, k}
    }

    pub fn multiply(&self, a: Vector3) -> Vector3{
        let x = self.i.multiply(a.x);
        let y = self.j.multiply(a.y);
        let z = self.k.multiply(a.z);
        x.add(y).add(z)
    }

    pub fn look_at_matrix(dir: Vector3) -> Self{
        let right = dir.cross(Vector3::new(0f32, 1f32, 0f32)).normalized().multiply(-1f32);
        let up = right.cross(dir).multiply(-1f32);
        Self::new(right, up, dir)
    }
}

