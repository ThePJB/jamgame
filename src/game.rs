use glow::HasContext;
use glow::*;
use glutin::event::VirtualKeyCode;
use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event::MouseButton;
use glutin::event::ElementState;
use std::time::Instant;
use crate::math::*;
use crate::vertex_buffer;
use crate::vertex_buffer::*;
use std::collections::HashSet;

// Vertex buffer: separate one for screen geometry as for world geometry
// yes because then we can do camera and zoom easily instead of it being completely fucked
// and it can also handle the aspect ratio

pub struct Game {
    xres: i32,
    yres: i32,
    window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    gl: glow::Context,
    
    mouse_pos: V2,
    held_keys: HashSet<VirtualKeyCode>,

    t: f32,
    t_last: Instant,

    program: glow::NativeProgram,
    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    texture: glow::NativeTexture,

    player_pos: V2,
    player_vel: V2,
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

        Game {
            xres,
            yres,
            window,
            gl,
            mouse_pos: v2(0., 0.),
            t: 0.0,
            t_last: Instant::now(),
            program,
            vao,
            vbo,
            texture,
            player_pos: v2(0., 0.),
            player_vel: v2(0., 0.),
            held_keys: HashSet::new(),
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
                    },
                    WindowEvent::Resized(size) => {
                        self.xres = size.width as i32;
                        self.yres = size.height as i32;
                        self.gl.viewport(0, 0, size.width as i32, size.height as i32)
                    },
                    WindowEvent::MouseInput {state: ElementState::Pressed, button: MouseButton::Left, ..} => {
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
                                    VirtualKeyCode::M => {
                                        println!("player pressed M");
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
        println!("frame");
        let t_now = Instant::now();
        let dt = (t_now - self.t_last).as_secs_f32();
        self.t += dt;
        self.t_last = t_now;

        let mut screen_geometry = VertexBuffer::new();
        let mut world_geometry = VertexBuffer::new();

        let screen_rect = v4(-1., -1., 2., 2.);
        let aspect = self.xres as f32 / self.yres as f32;

        self.simulate(dt);

        let cam_x = self.player_pos.x;
        let cam_y = self.player_pos.y;
        let x_scale = 1.0/aspect;   // either this or 1/aspect
        let y_scale = 1.0;

        screen_geometry.put_rect(screen_rect, v4(0., 0., 1., 1.), 0.9, v4(0.0, 0.0, 1.0, 1.0), 0);
        world_geometry.put_rect(v4(self.t.sin(), self.t.sin(), 1., 1.), v4(0., 0., 1., 1.), 0.8, v4(1., 0., 0., 1.), 0);
        world_geometry.put_rect(v4(0., 0., 1., 1.), v4(0., 0., 1., 1.), 0.8, v4(0., 1., 0., 1.), 0);

        self.gl.uniform_1_f32(self.gl.get_uniform_location(self.program, "time").as_ref(), self.t);

        self.gl.clear_color(0.5, 0.5, 0.5, 1.0);
        self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 
        self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
        self.gl.use_program(Some(self.program));
        self.gl.bind_vertex_array(Some(self.vao));
        self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));

        self.gl.uniform_matrix_4_f32_slice(self.gl.get_uniform_location(self.program, "projection").as_ref(), false, &[
            1., 0., 0., 0.,
            0., -1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ]);
        self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &screen_geometry.buf, glow::DYNAMIC_DRAW);
        let vert_count = screen_geometry.buf.len() / (10*4);
        self.gl.draw_arrays(glow::TRIANGLES, 0, vert_count as i32);
        

        self.gl.uniform_matrix_4_f32_slice(self.gl.get_uniform_location(self.program, "projection").as_ref(), true, &[
            x_scale, 0., 0., -cam_x,
            0., -1.*y_scale, 0., cam_y,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ]);
        self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &world_geometry.buf, glow::DYNAMIC_DRAW);
        let vert_count = world_geometry.buf.len() / (10*4);
        self.gl.draw_arrays(glow::TRIANGLES, 0, vert_count as i32);
        
        self.window.swap_buffers().unwrap();
    }

    pub fn simulate(&mut self, dt: f32) {
        let player_speed = 1.0;
        self.player_vel = v2(0., 0.);
        if self.held_keys.contains(&VirtualKeyCode::A) {
            self.player_vel.x -= player_speed;
        }
        if self.held_keys.contains(&VirtualKeyCode::D) {
            self.player_vel.x += player_speed;
        }
        if self.held_keys.contains(&VirtualKeyCode::W) {
            self.player_vel.y -= player_speed;
        }
        if self.held_keys.contains(&VirtualKeyCode::S) {
            self.player_vel.y += player_speed;
        }
        self.player_pos = self.player_pos + self.player_vel * dt;
    }
}