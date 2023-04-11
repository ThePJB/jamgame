use crate::math::*;

pub struct VertexBuffer {
    pub buf: Vec<u8>,
}

impl VertexBuffer {
    pub fn new() -> VertexBuffer {
        VertexBuffer {
            buf: Vec::new(),
        }
    }

    fn put_u32(&mut self, x: u32) {
        for b in x.to_le_bytes() {
            self.buf.push(b);
        }
    }

    fn put_float(&mut self, x: f32) {
        for b in x.to_le_bytes() {
            self.buf.push(b);
        }
    }

    pub fn put_vertex(&mut self, p: V3, uv: V2, col: V4, mode: u32) {
        self.put_float(p.x);
        self.put_float(p.y);
        self.put_float(p.z);
        self.put_float(col.x);
        self.put_float(col.y);
        self.put_float(col.z);
        self.put_float(col.w);
        self.put_float(uv.x);
        self.put_float(uv.y);
        self.put_u32(mode);
    }
    pub fn put_triangle(&mut self, p1: V2, uv1: V2, p2: V2, uv2: V2, p3: V2, uv3: V2, depth: f32, colour: V4, mode: u32) {
        self.put_vertex(v3(p1.x, p1.y, depth), uv1, colour, mode);
        self.put_vertex(v3(p2.x, p2.y, depth), uv2, colour, mode);
        self.put_vertex(v3(p3.x, p3.y, depth), uv3, colour, mode);
    }
    // 4 is top left
    pub fn put_quad(&mut self, p1: V2, uv1: V2, p2: V2, uv2: V2, p3: V2, uv3: V2, p4: V2, uv4: V2, depth: f32, colour: V4, mode: u32) {
        self.put_triangle(p1, uv1, p2, uv2, p3, uv3, depth, colour, mode);
        self.put_triangle(p3, uv3, p4, uv4, p1, uv1, depth, colour, mode);
    }

    pub fn put_rect(&mut self, r: V4, r_uv: V4, depth: f32, colour: V4, mode: u32) {
        self.put_triangle(r.tl(), r_uv.tl(), r.tr(), r_uv.tr(), r.bl(), r_uv.bl(), depth, colour, mode);
        self.put_triangle(r.bl(), r_uv.bl(), r.tr(), r_uv.tr(), r.br(), r_uv.br(), depth, colour, mode);
    }

    pub fn put_rect_flipx(&mut self, r: V4, r_uv: V4, depth: f32, colour: V4, mode: u32) {
        self.put_triangle(r.tl(), r_uv.tr(), r.tr(), r_uv.tl(), r.bl(), r_uv.br(), depth, colour, mode);
        self.put_triangle(r.bl(), r_uv.br(), r.tr(), r_uv.tl(), r.br(), r_uv.bl(), depth, colour, mode);
    }

    pub fn put_rect_flipy(&mut self, r: V4, r_uv: V4, depth: f32, colour: V4, mode: u32) {
        self.put_quad(r.tr(), r_uv.br(), r.br(), r_uv.tr(), r.bl(), r_uv.tl(), r.tl(), r_uv.bl(), depth, colour, mode);
    }
    pub fn put_rect_flipxy(&mut self, r: V4, r_uv: V4, depth: f32, colour: V4, mode: u32) {
        self.put_triangle(r.tl(), r_uv.br(), r.tr(), r_uv.bl(), r.bl(), r_uv.tr(), depth, colour, mode);
        self.put_triangle(r.bl(), r_uv.tr(), r.tr(), r_uv.bl(), r.br(), r_uv.tl(), depth, colour, mode);
    }

    pub fn put_glyph(&mut self, c: char, r: V4, depth: f32, colour: V4) {
        let clip_fn = |mut c: u8| {
            if c >= 'a' as u8 && c <= 'z' as u8 {
                c -= 'a' as u8 - 'A' as u8;
            }
            if c >= '+' as u8 && c <= '_' as u8 {
                let x = c - '+' as u8;
                let w = '_' as u8 - '+' as u8 + 1; // maybe +1
                Some(v4(0.0, 0.0, 1.0, 0.5).grid_child(x as usize, 0, w as usize, 1))
            } else {
                None
            }
        };
        if let Some(r_uv) = clip_fn(c as u8) {
            self.put_rect(r, r_uv, depth, colour, 1);
        }
    }

    pub fn put_sprite(&mut self, idx: usize, r: V4, depth: f32, colour: V4) {
        let r_uv = v4(0.0, 0.5, 40.0/39.75, 0.5).grid_child(idx as usize, 0, 40 as usize, 1);
        self.put_rect(r, r_uv, depth, colour, 1);
    }

    pub fn put_sprite_flipx(&mut self, idx: usize, r: V4, depth: f32, colour: V4) {
        let r_uv = v4(0.0, 0.5, 40.0/39.75, 0.5).grid_child(idx as usize, 0, 40 as usize, 1);
        self.put_rect_flipx(r, r_uv, depth, colour, 1);
    }

    pub fn put_sprite_flipy(&mut self, idx: usize, r: V4, depth: f32, colour: V4) {
        let r_uv = v4(0.0, 0.5, 40.0/39.75, 0.5).grid_child(idx as usize, 0, 40 as usize, 1);
        self.put_rect_flipy(r, r_uv, depth, colour, 1);
    }

    pub fn put_sprite_flipxy(&mut self, idx: usize, r: V4, depth: f32, colour: V4) {
        let r_uv = v4(0.0, 0.5, 40.0/39.75, 0.5).grid_child(idx as usize, 0, 40 as usize, 1);
        self.put_rect_flipxy(r, r_uv, depth, colour, 1);
    }

    pub fn put_string_left(&mut self, s: &str, mut x: f32, y: f32, cw: f32, ch: f32, depth: f32, colour: V4) {
        for c in s.chars() {
            self.put_glyph(c, v4(x, y, cw, ch), depth, colour);
            x += cw;
        }
    }
    pub fn put_string_centered(&mut self, s: &str, mut x: f32, mut y: f32, cw: f32, ch: f32, depth: f32, colour: V4) {
        let w = s.len() as f32 * cw;
        x -= w/2.0;
        // y -= ch/2.0;
        for c in s.chars() {
            self.put_glyph(c, v4(x, y, cw, ch), depth, colour);
            x += cw;
        }
    }
}