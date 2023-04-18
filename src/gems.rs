use crate::game::*;
use crate::math::*;

const COLLECTION_RADIUS: f32 = 0.08;

impl Game {
    pub fn draw_gems(&mut self) {
        for i in 0..self.gem_x.len() {
            let gem_s = 0.1;
            let seed = khash(i * 1231247 + 21321377);

            let s = match self.gem_type[i] {
                0 => 1.0,
                1 => 1.5,
                2 => 2.0,
                3 => 2.5,
                _ => {panic!("impossible")},
            };

            let colour = match self.gem_type[i] {
                0 => v4(1., 0., 0., 1.),
                1 => v4(0., 1., 0., 1.),
                2 => v4(0., 0., 1., 1.),
                3 => v4(1., 0., 1., 1.),
                _ => {panic!("impossible")},
            };

            let yo = ((krand(seed) * 2.0 * PI) + self.t).sin() * 0.04 + 0.1;

            self.world_geometry.put_rect(v2(self.gem_x[i], self.gem_y[i] + yo * s).rect_centered(gem_s * s * 0.8, gem_s * s * 0.5 * 0.8), v4(0., 0., 1., 1.,), 0.4 + 0.01, colour, 3);
            self.world_geometry.put_rect(v2(self.gem_x[i], self.gem_y[i]).rect_centered(gem_s * s, gem_s * s), v4(0., 0., 1., 1.,), 0.4, colour, 2);
        }
    }

    
    pub fn update_gems(&mut self, dt: f32) {
        let mut idx = self.gem_x.len();
        while idx > 0 {
            idx -= 1;

            let dx = self.player_pos.x - self.gem_x[idx];
            let dy = self.player_pos.y - self.gem_y[idx];
            let d = (dx*dx+dy*dy).sqrt();

            let accel = 16.0;
            let fric = 4.0;

            if d < self.player_succ {
                self.gem_vx[idx] += accel * dx.signum()*(self.player_succ - d) * dt;
                self.gem_vy[idx] += accel * dy.signum()*(self.player_succ - d) * dt;
            }
            self.gem_x[idx] += self.gem_vx[idx] * dt;
            self.gem_y[idx] += self.gem_vy[idx] * dt;

            self.gem_vx[idx] -= self.gem_vx[idx] * dt * fric;
            self.gem_vy[idx] -= self.gem_vy[idx] * dt * fric;

            if d < COLLECTION_RADIUS {
                self.gems += match self.gem_type[idx] {
                    0 => 10,
                    1 => 20,
                    2 => 50,
                    3 => 150,
                    _ => panic!("impossible"),
                };

                self.gem_type.swap_remove(idx);
                self.gem_x.swap_remove(idx);
                self.gem_y.swap_remove(idx);
                self.gem_vx.swap_remove(idx);
                self.gem_vy.swap_remove(idx);

            }
        }
    }

    pub fn populate_with_gems(&mut self) {
        // world variability: noise scale
        // richness multi
        // richness offset


        let n = 100;
        let nscale = 0.3;
        let gw = LEVEL_W / n as f32;
        let gh = LEVEL_H / n as f32;
        for i in 0..n {
            for j in 0..n {
                let si = khash(self.level_seed + i * 1231234517 + j * 120312497 + 12312767);
                if krand(si * 1231237) < 0.9 {continue;}
                let oy = gh * krand(si * 12312377);
                let ox = gw * krand(si * 12301297);
                let x = i as f32 * gw + ox - LEVEL_W/2.0;
                let y = j as f32 * gh + oy - LEVEL_H/2.0;
                let richness = 1.7 * noise2(x * nscale, y * nscale, (self.level_seed * 12312397) as u32) - 0.3;
                let roll = krand(si);

                if richness < 0.5 {
                    if roll * 5.0 < richness {
                        self.gem_x.push(x);
                        self.gem_y.push(y);
                        self.gem_vx.push(0.0);
                        self.gem_vy.push(0.0);
                        self.gem_type.push(0);
                    }
                    continue;
                }

                if richness > 0.9 {
                    if roll > 0.9 {
                        self.gem_x.push(x);
                        self.gem_y.push(y);
                        self.gem_type.push(3);
                        self.gem_vx.push(0.0);
                        self.gem_vy.push(0.0);
                        continue;
                    } else if roll > 0.8 {
                        self.gem_x.push(x);
                        self.gem_y.push(y);
                        self.gem_type.push(2);
                        self.gem_vx.push(0.0);
                        self.gem_vy.push(0.0);
                    } else {
                        self.gem_x.push(x);
                        self.gem_y.push(y);
                        self.gem_type.push(1);
                        self.gem_vx.push(0.0);
                        self.gem_vy.push(0.0);
                    } 
                    continue;
                }

                if richness > 0.75 {
                    if roll > 0.9 {
                        self.gem_x.push(x);
                        self.gem_y.push(y);
                        self.gem_type.push(2);
                        self.gem_vx.push(0.0);
                        self.gem_vy.push(0.0);
                    } else if roll > 0.8 {
                        self.gem_x.push(x);
                        self.gem_y.push(y);
                        self.gem_type.push(1);
                        self.gem_vx.push(0.0);
                        self.gem_vy.push(0.0);
                    } else {
                        self.gem_x.push(x);
                        self.gem_y.push(y);
                        self.gem_type.push(0);
                        self.gem_vx.push(0.0);
                        self.gem_vy.push(0.0);
                    }
                    continue;
                }

                if roll > 0.9 {
                    self.gem_x.push(x);
                    self.gem_y.push(y);
                    self.gem_type.push(1);
                    self.gem_vx.push(0.0);
                    self.gem_vy.push(0.0);
                } else {
                    self.gem_x.push(x);
                    self.gem_y.push(y);
                    self.gem_type.push(0);
                    self.gem_vx.push(0.0);
                    self.gem_vy.push(0.0);
                }
            }
        }
    }
}