use glow::HasContext;
use glow::*;
use glutin::event::VirtualKeyCode;
use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event::MouseButton;
use glutin::event::ElementState;
use std::time::Instant;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use crate::math::*;
use crate::vertex_buffer::*;
use std::collections::HashSet;
pub use std::f32::consts::PI;

use crate::gems;
use crate::enemies;
use crate::player;


pub const LEVEL_W: f32 = 20.0;
pub const LEVEL_H: f32 = 20.0;

pub struct Game {
    pub xres: i32,
    pub yres: i32,
    pub window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    pub gl: glow::Context,
    
    pub mouse_pos: V2,
    pub aim: V2,
    pub lmb: bool,
    pub held_keys: HashSet<VirtualKeyCode>,

    pub t: f32,
    pub t_last: Instant,
    pub t_level: f32,

    pub program: glow::NativeProgram,
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,
    pub texture: glow::NativeTexture,

    pub screen_geometry: VertexBuffer,
    pub world_geometry: VertexBuffer,

    pub in_portal: bool,

    pub player_pos: V2,
    pub player_vel: V2,
    pub player_succ: f32,
    pub player_hp: f32,
    pub player_hp_regen: f32,
    pub player_speed: f32,
    pub player_t_shoot: f32,
    pub player_cooldown: f32,
    pub player_damage: f32,
    pub player_proj_speed: f32,

    pub seed: usize,
    pub level_seed: usize,
    pub enemies_spawn_seed: usize,

    pub gem_x: Vec<f32>,
    pub gem_y: Vec<f32>,
    pub gem_vx: Vec<f32>,
    pub gem_vy: Vec<f32>,
    pub gem_type: Vec<usize>,

    pub gems: usize,

    pub spawn_enemies_counter: f32,

    pub enemy_x: Vec<f32>,
    pub enemy_y: Vec<f32>,
    pub enemy_type: Vec<usize>,
    pub enemy_birth_t: Vec<f32>,
    pub enemy_attack_t: Vec<f32>,
    pub enemy_hp: Vec<f32>,
    
    pub enemy_projectile_x: Vec<f32>,
    pub enemy_projectile_y: Vec<f32>,
    pub enemy_projectile_vx: Vec<f32>,
    pub enemy_projectile_vy: Vec<f32>,
    pub enemy_projectile_type: Vec<usize>,
    
    pub player_projectile_x: Vec<f32>,
    pub player_projectile_y: Vec<f32>,
    pub player_projectile_vx: Vec<f32>,
    pub player_projectile_vy: Vec<f32>,

}

impl Game {
    pub unsafe fn new(event_loop: &glutin::event_loop::EventLoop<()>) -> Game {
        let xres = 800i32;
        let yres = 800i32;
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Wizrad 4")
            .with_inner_size(glutin::dpi::PhysicalSize::new(xres, yres));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        gl.enable(DEPTH_TEST);
        // gl.enable(CULL_FACE);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        gl.depth_func(LEQUAL);
        gl.enable(BLEND);
        // gl.debug_message_callback(|a, b, c, d, msg| {
        //     println!("{} {} {} {} msg: {}", a, b, c, d, msg);
        // });

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 4*4 + 4*3 + 4*2 + 4, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 4*4 + 4*3 + 4*2 + 4, 4*3);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 4*4 + 4*3 + 4*2 + 4, 4*3 + 4*4);
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_pointer_i32(3, 1, glow::UNSIGNED_INT, 4*4 + 4*3 + 4*2 + 4, 4*3 + 4*4 + 4*2);
        gl.enable_vertex_attrib_array(3);


        // Shader
        let program = gl.create_program().expect("Cannot create program");
    
        let vs = gl.create_shader(glow::VERTEX_SHADER).expect("cannot create vertex shader");
        gl.shader_source(vs, include_str!("shader.vert"));
        gl.compile_shader(vs);
        if !gl.get_shader_compile_status(vs) {
            panic!("{}", gl.get_shader_info_log(vs));
        }
        gl.attach_shader(program, vs);

        let fs = gl.create_shader(glow::FRAGMENT_SHADER).expect("cannot create fragment shader");
        gl.shader_source(fs, include_str!("shader.frag"));
        gl.compile_shader(fs);
        if !gl.get_shader_compile_status(fs) {
            panic!("{}", gl.get_shader_info_log(fs));
        }
        gl.attach_shader(program, fs);

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }
        gl.detach_shader(program, fs);
        gl.delete_shader(fs);
        gl.detach_shader(program, vs);
        gl.delete_shader(vs);

        let png_bytes = include_bytes!("../tex.png").as_ref();
        let decoder = png::Decoder::new(png_bytes);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        let bytes = &buf[..info.buffer_size()];

        let texture = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_image_2d(
            glow::TEXTURE_2D, 
            0, 
            glow::RGBA as i32, 
            info.width as i32, info.height as i32, 
            0, 
            RGBA, 
            glow::UNSIGNED_BYTE, 
            Some(bytes)
        );
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

        gl.generate_mipmap(glow::TEXTURE_2D);

        let initial_seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as usize;

        Game {
            xres,
            yres,
            window,
            gl,
            mouse_pos: v2(0., 0.),
            aim: v2(0., 0.),
            lmb: false,
            t: 0.0,
            t_level: 100.0,
            t_last: Instant::now(),
            program,
            vao,
            vbo,
            texture,
            in_portal: false,
            player_pos: v2(0., 0.),
            player_vel: v2(0., 0.),
            player_succ: 1.0,
            player_hp: 1.0,
            player_hp_regen: 0.05,
            player_speed: 1.0,
            player_t_shoot: -999.0, 
            player_cooldown: 0.7,
            player_damage: 0.5,
            player_proj_speed: 2.0,
            held_keys: HashSet::new(),
            screen_geometry: VertexBuffer::default(),
            world_geometry: VertexBuffer::default(),
            seed: initial_seed,
            enemies_spawn_seed: khash(initial_seed * 1231247),
            level_seed: khash(initial_seed * 129371237),
            gem_type: vec![],
            gem_x: vec![],
            gem_y: vec![],
            gem_vx: vec![],
            gem_vy: vec![],
            gems: 0,
            spawn_enemies_counter: 0.0,
            enemy_x: vec![],
            enemy_y: vec![],
            enemy_type: vec![],
            enemy_birth_t: vec![],
            enemy_attack_t: vec![],
            enemy_hp: vec![],
            enemy_projectile_x: vec![],
            enemy_projectile_y: vec![],
            enemy_projectile_vx: vec![],
            enemy_projectile_vy: vec![],
            enemy_projectile_type: vec![],
            player_projectile_x: vec![],
            player_projectile_y: vec![],
            player_projectile_vx: vec![],
            player_projectile_vy: vec![],
        }
    }

    pub unsafe fn handle_event(&mut self, event: Event<()>) {
        match event {
            Event::LoopDestroyed |
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                std::process::exit(0);
            }

            Event::WindowEvent {event, .. } => {
                match event {
                    WindowEvent::CursorMoved {position, .. } => {
                        self.mouse_pos.x = position.x as f32 / self.xres as f32;
                        self.mouse_pos.y = position.y as f32 / self.yres as f32;
                        self.aim.x = (position.x as f32 - self.xres as f32/2.0) as f32 / self.xres as f32;
                        self.aim.y = (position.y as f32 - self.yres as f32/2.0) as f32 / self.yres as f32;
                        self.aim.x *= (self.xres as f32 / self.yres as f32);
                        self.aim = self.aim.normalize();
                        dbg!(self.xres, self.yres);
                    },
                    WindowEvent::Resized(size) => {
                        self.xres = size.width as i32;
                        self.yres = size.height as i32;
                        self.gl.viewport(0, 0, size.width as i32, size.height as i32)
                    },
                    WindowEvent::MouseInput {state: ElementState::Pressed, button: MouseButton::Left, ..} => {
                        self.lmb = true;
                    },
                    WindowEvent::MouseInput {state: ElementState::Released, button: MouseButton::Left, ..} => {
                        self.lmb = false;
                    },
                    WindowEvent::KeyboardInput {input, ..} => {
                        match input {
                            glutin::event::KeyboardInput {virtual_keycode: Some(code), state: ElementState::Pressed, ..} => {
                                self.held_keys.insert(code);
                            },
                            glutin::event::KeyboardInput {virtual_keycode: Some(code), state: ElementState::Released, ..} => {
                                self.held_keys.remove(&code);
                                match code {
                                    VirtualKeyCode::Escape => {
                                    },
                                    VirtualKeyCode::T => {
                                        if !self.in_portal && self.gems > 500 {
                                            self.gems -= 500;
                                            self.in_portal = true;
                                            self.zero_state();
                                        }
                                        println!("portal");
                                    },
                                    _ => {},
                                }
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            Event::MainEventsCleared => self.frame(),
            _ => {},
        }
    }

    pub unsafe fn frame(&mut self) {
        let t_now = Instant::now();
        let dt = (t_now - self.t_last).as_secs_f32();
        self.t += dt;
        self.t_last = t_now;

        self.screen_geometry = VertexBuffer::default();
        self.world_geometry = VertexBuffer::default();

        let screen_rect = v4(-1., -1., 2., 2.);
        let aspect = self.xres as f32 / self.yres as f32;

        self.simulate(dt);

        let cam_x = self.player_pos.x;
        let cam_y = self.player_pos.y;
        let scale = 1.0;
        let x_scale = scale/aspect;   // either this or 1/aspect
        let y_scale = scale;
        let fntsize = 0.1;

        
        if self.in_portal {
            self.screen_geometry.put_rect(v4(-1., -1., 2., 2.), v4(0., 0., 1., 1.,), 0.9, v4(0., 0., 0., 1.), 0);
            self.screen_geometry.put_string_left(&format!("gems: {}", self.gems),  -1.0, -1.0, 12./16. * fntsize, fntsize, 0.1, v4(1., 1., 1., 1.));
            if self.t % 2.0 < 1.0 {
                self.screen_geometry.put_string_centered("-- WARP ZONE --",  0.0, 1.0-fntsize, 12./16. * fntsize, fntsize, 0.1, v4(1., 1., 1., 1.));
            }

            // draw cool white lines
            for i in 0..20 {
                let t_lines = self.t * 2.0;
                let si = khash(self.seed + i * 1293124147);
                let phase_i = krand(si) * (2.0*PI);
                let cycle_i = ((t_lines - phase_i) / (2.0*PI)).floor() as usize;
                let sc = khash(si + 1231237 * cycle_i);
                let x = krand(sc) * 2.0 - 1.0;
                let phase = (t_lines - phase_i) % (2.0*PI);
                let y = (phase / (2.0*PI)) * 2.0 - 1.0;
                self.screen_geometry.put_rect(v4(x, y, 0.01, 0.1), v4(0., 0., 1., 1.,), 0.3, v4(1., 1., 1., 1.), 0);
            }


        } else {
            // normal level drawing
            self.draw_level();
            self.draw_player();
            self.draw_gems();
            self.draw_enemies();
            self.draw_enemy_projectiles();
            self.draw_player_projectiles();

            // draw gem count
            self.screen_geometry.put_string_left(&format!("hp: {} gems: {}",(self.player_hp * 100.0).round(),  self.gems),  -1.0, -1.0, 12./16. * fntsize, fntsize, 0.1, v4(1., 1., 1., 1.));
        }



        self.gl.uniform_1_f32(self.gl.get_uniform_location(self.program, "time").as_ref(), self.t);

        self.gl.clear_color(0.5, 0.5, 0.5, 1.0);
        self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 
        self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
        self.gl.use_program(Some(self.program));
        self.gl.bind_vertex_array(Some(self.vao));
        self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));

        self.gl.uniform_matrix_4_f32_slice(self.gl.get_uniform_location(self.program, "projection").as_ref(), true, &[
            x_scale, 0., 0., -cam_x*x_scale,
            0., -1.*y_scale, 0., cam_y*y_scale,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ]);
        self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &self.world_geometry.buf, glow::DYNAMIC_DRAW);
        let vert_count = self.world_geometry.buf.len() / (10*4);
        self.gl.draw_arrays(glow::TRIANGLES, 0, vert_count as i32);

        self.gl.uniform_matrix_4_f32_slice(self.gl.get_uniform_location(self.program, "projection").as_ref(), true, &[
            1./aspect, 0., 0., 1./aspect - 1.0,
            0., -1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ]);
        self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &self.screen_geometry.buf, glow::DYNAMIC_DRAW);
        let vert_count = self.screen_geometry.buf.len() / (10*4);
        self.gl.draw_arrays(glow::TRIANGLES, 0, vert_count as i32);
        
        self.window.swap_buffers().unwrap();
    }

    pub fn simulate(&mut self, dt: f32) {
        self.spawn_enemies_counter += dt;
        self.t_level += dt;

        self.update_player(dt);
        self.update_player_projectiles(dt);
        self.update_gems(dt);

        if self.spawn_enemies_counter > 1.0 {
            self.spawn_enemies_counter -= 1.0;
            self.spawn_enemies();
        }

        self.update_enemies(dt);
        self.update_enemy_projectiles(dt);
    }

    pub fn draw_level(&mut self) {
        let world_colour = v4(50.0, 0.3, 0.9, 1.0).hsv_to_rgb();
        // we could do some texturing on gpu
        // or just place rocks and shit as well as da gems
        self.world_geometry.put_rect(v4(-LEVEL_W/2.0, -LEVEL_H/2.0, LEVEL_W, LEVEL_H,), v4(0., 0., 1., 1.), 0.9, world_colour, 0);
    }    

    pub fn zero_state(&mut self) {
        self.enemy_x = vec![];
        self.enemy_y = vec![];
        self.enemy_type = vec![];
        self.enemy_birth_t = vec![];
        self.enemy_attack_t = vec![];
        self.enemy_hp = vec![];
        self.enemy_projectile_x = vec![];
        self.enemy_projectile_y = vec![];
        self.enemy_projectile_vx = vec![];
        self.enemy_projectile_vy =  vec![];
        self.enemy_projectile_type = vec![];
        self.player_projectile_x = vec![];
        self.player_projectile_y = vec![];
        self.player_projectile_vx = vec![];
        self.player_projectile_vy = vec![];
    }
}