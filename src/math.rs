pub fn khash(mut state: usize) -> usize {
    state = (state ^ 2747636419).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state
}
pub fn khashu32(mut state: u32) -> u32 {
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
    pub fn rect_centered(&self, w: f32, h: f32) -> V4 {
        v4(self.x-w/2.0, self.y-h/2.0, w, h)
    }
    pub fn lerp(&self, other: V2, t: f32) -> V2 {
        v2(lerp(self.x, other.x, t), lerp(self.y, other.y, t))
    }
    pub fn homogeneous_transform(&self, mat: &[f32; 9]) -> V2 {
        v2(
            mat[0] * self.x + mat[1] * self.y + mat[2],
            mat[3] * self.x + mat[4] * self.y + mat[5],
        )
    }
    pub fn norm(&self) -> f32 {
        let mag = (self.x*self.x + self.y*self.y).sqrt();
        return mag;
    }
    pub fn normalize(&self) -> V2 {
        let mag = (self.x*self.x + self.y*self.y).sqrt();
        if mag == 0.0 {
            return v2(0.0, 0.0);
        }

        *self / mag
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

const ROOT3ON2: f32 = 0.8660254037844386467637231707529361834714026269051903140279034897;
const ROOT2INV: f32 = 0.70710678118;

pub fn khash_float2(seed: u32, x: f32, y: f32) -> u32 {
    let x_u32: u32 = bytemuck::cast(x);
    let y_u32: u32 = bytemuck::cast(y);
    khashu32(seed.wrapping_add(x_u32.wrapping_mul(0x548AB4C9).wrapping_add(y_u32.wrapping_mul(0x97124DA8))))
}
fn bilinear(a: f32, b: f32, c: f32, d: f32, t1: f32, t2: f32) -> f32 {
    //let u = |x| x*x*(3.0-2.0*x);
    //let u = |x| x*x*(10.0-3.0*x*(5.0-2.0*x));                 // looks fucked, directional artifacts I wonder why. maybe because derivative is discontinuous in middle
    //let u = |x: f32| (std::f32::consts::FRAC_PI_2*x).sin();   // looks fucked, I expected better from you sin, maybe derivative discontinuous in middle
    //let u = |x| x;
    let u = |x| ((6.0*x - 15.0)*x + 10.0)*x*x*x;
    lerp(lerp(a, b, u(t1)), lerp(c, d, u(t1)), u(t2))
}
pub fn floorfrac(x: f32) -> (f32, f32) {
    let floor = x.floor();
    if x < 0.0 {
        (floor, (floor - x).abs())
    } else {
        (floor, x - floor)
    }
}
pub fn noise2(x: f32, y: f32, seed: u32) -> f32 {
    let (xfloor, xfrac) = floorfrac(x);
    let (yfloor, yfrac) = floorfrac(y);
    // also why not use a bigger gradient table
    //let grads = [(-1.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (root2, root2), (-root2, root2), (root2, -root2), (-root2, -root2)];
    let grads = [(-1.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (ROOT2INV, ROOT2INV), (-ROOT2INV, ROOT2INV), (ROOT2INV, -ROOT2INV), (-ROOT2INV, -ROOT2INV),
        (0.5, ROOT3ON2), (0.5, ROOT3ON2), (-0.5, -ROOT3ON2), (-ROOT3ON2, -0.5), (-0.5, ROOT3ON2), (-ROOT3ON2, 0.5), (0.5, -ROOT3ON2), (ROOT3ON2, -0.5)
    ];
    // idk whystefan gustavson does the below and not the above. it kinda does look better lol
    // also why not more gradients?
    //let grads = [(-1.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)];

    let cf = |corner_x: f32, corner_y: f32| {
        let g_idx = khash_float2(seed, corner_x + xfloor, corner_y + yfloor) & 15;
        //let g_idx = khash(xu + corner_x + (yu + corner_y) * 0xA341316C + seed * 0xF73DB187) & 15;
        let (dx, dy) = grads[g_idx as usize];
        // println!("dx {} dy {}", dx, dy);
        // println!("xfrac {} yfrac {}", x.fract(), y.fract());
        dx * (xfrac - corner_x as f32) + dy * (yfrac - corner_y as f32)
    };
    
    let c1 = cf(0.0,0.0);
    let c2 = cf(1.0,0.0);
    let c3 = cf(0.0,1.0);
    let c4 = cf(1.0,1.0);

    let result = bilinear(c1, c2, c3, c4, xfrac, yfrac);
    // println!("c1 {} c2 {} c3 {} c4 {}", c1, c2, c3, c4);
    // println!("res: {}", result);
    (result + 1.0) / 2.0
}

pub fn fnoise2(x: f32, y: f32, seed: u32) -> f32 {
    (1.000 * noise2(x, y, seed.wrapping_mul(0x3523423)) +
    0.500 * noise2(x * 2.0, y * 2.0, seed.wrapping_mul(0xF73DB187)) + 
    0.250 * noise2(x * 4.0, y * 4.0, seed.wrapping_mul(0x159CBAFE)) + 
    0.125 * noise2(x * 8.0, y * 8.0, seed.wrapping_mul(0x83242364))) /
    1.675
}