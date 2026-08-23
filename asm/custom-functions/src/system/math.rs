use core::f32::consts;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vec3s {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Matrix34f {
    pub m: [[f32; 4]; 3],
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Matrix44f {
    pub m: [[f32; 4]; 4],
}

extern "C" {
    pub fn C_MTXOrtho(
        m: *mut Matrix44f,
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
        near: f32,
        far: f32,
    );
    pub fn PSMTXIdentity(mtx: *mut Matrix34f);
    fn EGGSqrt(x: f32) -> f32;
    fn EGGSin(ang: f32) -> f32;
    fn EGGCos(ang: f32) -> f32;
    fn EGGAcos(x: f32) -> f32;
    fn EGGAtan2(y: f32, x: f32) -> f32;
}

// helper math functions

pub fn sqrt(x: f32) -> f32 {
    unsafe { EGGSqrt(x) }
}

pub fn sin(ang: f32) -> f32 {
    unsafe { EGGSin(ang) }
}

pub fn cos(ang: f32) -> f32 {
    unsafe { EGGCos(ang) }
}

pub fn acos(x: f32) -> f32 {
    unsafe { EGGAcos(x) }
}

pub fn atan2(y: f32, x: f32) -> f32 {
    unsafe { EGGAtan2(y, x) }
}

const MIN_ANGLE: f32 = consts::TAU / 65536f32;

pub fn short_to_rad(ang: i16) -> f32 {
    (ang as f32) * MIN_ANGLE
}

pub fn normalize_angle(mut angle: f32) -> f32 {
    while angle >= consts::TAU {
        angle -= consts::TAU;
    }
    while angle < 0.0 {
        angle += consts::TAU;
    }
    angle
}

pub fn rad_to_short(angle: f32) -> i16 {
    let wrapped = normalize_angle(angle);
    let turns = wrapped / consts::TAU;
    ((turns * 65536.0) as u16) as i16
}

pub fn rad_to_deg(ang: f32) -> f32 {
    ang * 180f32 * consts::FRAC_1_PI
}

impl Vec3f {
    pub const fn zero() -> Self {
        Vec3f { x: 0.0, y: 0.0, z: 0.0 }
    }
    
    pub fn normalize(&mut self) {
        let len = self.len();
        if len >= 0.00001f32 {
            self.mul(1.0f32 / len);
        }
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        sqrt(self.len_squared())
    }

    pub fn dot(a: &Self, b: &Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: &Self, b: &Self) -> Self {
        let x = a.y * b.z - a.z * b.y;
        let y = a.z * b.x - a.x * b.z;
        let z = a.x * b.y - a.y * b.x;
        Self { x, y, z }
    }

    pub fn from_short(ang: i16) -> Self {
        let rad = short_to_rad(ang);
        let x = sin(rad);
        let z = cos(rad);
        Self {
            x, y: 0f32, z
        }
    }

    pub fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn sub(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    pub fn mul(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}
