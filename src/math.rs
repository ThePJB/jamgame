pub fn khash(mut state: usize) -> usize {
    state = (state ^ 2747636419).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state
}
pub fn krand(seed: usize) -> f32 {
    (khash(seed)&0x00000000FFFFFFFF) as f32 / 4294967295.0
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

#[derive(Clone, Copy, Debug)]
pub struct V2 {
    pub x: f32,
    pub y: f32,
}
pub fn v2(x: f32, y: f32) -> V2 { V2 { x, y } }
#[derive(Clone, Copy, Debug)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub fn v3(x: f32, y: f32, z: f32) -> V3 { V3 { x, y, z } }
#[derive(Clone, Copy, Debug)]
pub struct V4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub fn v4(x: f32, y: f32, z: f32, w: f32) -> V4 { V4 { x, y, z, w } }

impl V2 {
    pub fn dot(&self, other: V2) -> f32 {
        self.x*other.x + self.y * other.y
    }
}
impl V3 {
    pub fn dot(&self, other: V3) -> f32 {
        self.x*other.x + self.y * other.y + self.z*other.z
    }
}
impl V4 {
    pub fn dot(&self, other: V4) -> f32 {
        self.x*other.x + self.y * other.y + self.z*other.z + self.w*other.w
    }
    pub fn tl(&self) -> V2 {v2(self.x, self.y)}
    pub fn br(&self) -> V2 {v2(self.x + self.z, self.y + self.w)}
    pub fn tr(&self) -> V2 {v2(self.x + self.z, self.y)}
    pub fn bl(&self) -> V2 {v2(self.x, self.y + self.w)}
    pub fn grid_child(&self, i: usize, j: usize, w: usize, h: usize) -> V4 {
        let cw = self.z / w as f32;
        let ch = self.w / h as f32;
        v4(self.x + cw * i as f32, self.y + ch * j as f32, cw, ch)
    }
    pub fn hsv_to_rgb(&self) -> V4 {
        let v = self.z;
        let hh = (self.x % 360.0) / 60.0;
        let i = hh.floor() as i32;
        let ff = hh - i as f32;
        let p = self.z * (1.0 - self.y);
        let q = self.z * (1.0 - self.y * ff);
        let t = self.z * (1.0 - self.y * (1.0 - ff));
        match i {
            0 => v4(v, t, p, self.w),
            1 => v4(q, v, p, self.w),
            2 => v4(p, v, t, self.w),
            3 => v4(p, q, v, self.w),
            4 => v4(t, p, v, self.w),
            5 => v4(v, p, q, self.w),
            _ => panic!("unreachable"),
        }
    }
    fn contains(&self, p: V2) -> bool {
        !(p.x < self.x || p.x > self.x + self.z || p.y < self.y || p.y > self.y + self.w)
    }
    fn point_within(&self, p: V2) -> V2 {
        v2(p.x*self.z+self.x, p.y*self.w+self.y)
    }
    fn point_without(&self, p: V2) -> V2 {
        v2((p.x - self.x) / self.z, (p.y - self.y) / self.w)
    }
    fn fit_aspect(&self, a: f32) -> V4 {
        let a_self = self.z/self.w;

        if a_self > a {
            // parent wider
            v4((self.z - self.z*(1.0/a))/2.0, 0.0, self.z*1.0/a, self.w)
        } else {
            // child wider
            v4(0.0, (self.w - self.w*(1.0/a))/2.0, self.z, self.w*a)
        }
    }
}

impl std::ops::Sub<V2> for V2 {
    type Output = V2;

    fn sub(self, _rhs: V2) -> V2 {
        V2 { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}

impl std::ops::Add<V2> for V2 {
    type Output = V2;

    fn add(self, _rhs: V2) -> V2 {
        V2 { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl std::ops::Mul<f32> for V2 {
    type Output = V2;

    fn mul(self, _rhs: f32) -> V2 {
        V2 { x: self.x * _rhs, y: self.y * _rhs }
    }
}

impl std::ops::Mul<V2> for f32 {
    type Output = V2;

    fn mul(self, _rhs: V2) -> V2 {
        V2 { x: self * _rhs.x, y: self * _rhs.y }
    }
}

impl std::ops::Div<f32> for V2 {
    type Output = V2;

    fn div(self, _rhs: f32) -> V2 {
        V2 { x: self.x / _rhs, y: self.y / _rhs }
    }
}

impl std::ops::Neg for V2 {
    type Output = V2;

    fn neg(self) -> V2 {
        V2 { x: -1.0 * self.x, y: -1.0 * self.y}
    }
}