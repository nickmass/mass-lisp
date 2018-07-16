use glium::{self, glutin, Surface};

use std::collections::HashSet;
use std::cell::Cell;

use super::gfx::*;
use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Matrix3<T>(pub Vector3<T>, pub Vector3<T>, pub Vector3<T>);

impl Matrix3<f32> {
    pub fn identity() -> Matrix3<f32> {
        Matrix3(
            Vector3(1.0, 0.0, 0.0),
            Vector3(0.0, 1.0, 0.0),
            Vector3(0.0, 0.0, 1.0)
        )
    }
}

unsafe impl glium::vertex::Attribute for Vector2<f32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32
    }
}

unsafe impl glium::vertex::Attribute for Vector3<f32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32F32
    }
}

unsafe impl glium::vertex::Attribute for Matrix3<f32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x3x3
    }
}

impl glium::uniforms::AsUniformValue for Matrix3<f32> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        glium::uniforms::UniformValue::Mat3([self.0.as_array(), self.1.as_array(), self.2.as_array()])
    }
}

implement_vertex!(Vertex, position, color);

const SOLID_VERTEX_SHADER: &'static str = r#"
#version 110
uniform mat3 matrix;

attribute vec2 position;
attribute vec3 color;

varying vec3 vColor;

void main() {
    vec3 pos = vec3(position, 1.0) * matrix;
    gl_Position = vec4(pos.x / pos.z, pos.y/ pos.z, 0.0, 1.0);
    vColor = color;
}
"#;

const SOLID_FRAG_SHADER: &'static str = r#"
#version 110
varying vec3 vColor;

void main() {
    gl_FragColor = vec4(vColor, 1.0);
}
"#;

fn create_circle(segments: u32) -> Vec<Vector2<f32>> {
    let inc = 2.0 * ::std::f32::consts::PI / ((segments - 1) as f32);
    let mut acc = 0.0f32;
    let mut points = vec![Vector2(0.0, 0.0)];

    for _ in 0..segments {
        points.push(Vector2(acc.sin(), acc.cos()));
        acc += inc;
    }

    points
}

pub fn create_window(ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();

    let to_dim = |obj: Option<Object>| {
        let obj = obj
            .map(|o| core::integer_impl(&o))
            .unwrap_or(Number::Integer(512));

        if let Number::Integer(num) = obj {
            num as u32
        } else {
            512
        }
    };

    let (dim_x, dim_y) = (to_dim(args.next()), to_dim(args.next()));
    let callback = args.next().unwrap_or(Object::Nil);

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions((dim_x, dim_y).into());
    let context = glutin::ContextBuilder::new();
    let mut inputs = HashSet::new();
    if let Ok(display) = glium::Display::new(window, context, &events_loop) {
        let (dim_x, dim_y) = (dim_x as f32, dim_y as f32);

        let mut screen_scale = Matrix3::<f32>::identity();
        (screen_scale.0).0 = 2.0 / dim_x;
        (screen_scale.1).1 = -2.0 / dim_y;
        (screen_scale.0).2 = -1.0;
        (screen_scale.1).2 = 1.0;

        let circle_model = create_circle(32);

        let solid_program = glium::Program::from_source(&display, SOLID_VERTEX_SHADER, SOLID_FRAG_SHADER, None);
        if solid_program.is_err() {
            return Exception::message(format!("Unable to create GL program: {}", solid_program.unwrap_err())).into();
        }
        let solid_program = solid_program.unwrap();
        let mut line_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            line_width: Some(5.0),
            polygon_mode: glium::PolygonMode::Line,
            smooth: Some(glium::Smooth::Nicest),
            .. Default::default()
        };
        let fill_params = glium::DrawParameters {
            polygon_mode: glium::PolygonMode::Fill,
            .. Default::default()
        };

        let line_index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
        let circle_index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

        let mut close = Cell::new(false);
        let mut mouse_position = Cell::new((0.0, 0.0));
        let mut window_size = Cell::new((0.0, 0.0));

        display.gl_window().get_inner_size()
            .map(|s| window_size.set(s.into()));
        display.gl_window().hide_cursor(true);

        while !close.get() {
            events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => close.set(true),
                        glutin::WindowEvent::Resized(size) => {
                            window_size.set(size.into());
                        },
                        glutin::WindowEvent::CursorMoved{ position, .. } => {
                            let (x, y): (f64, f64) = position.into();
                            let (win_x, win_y) = window_size.get();
                            let (x, y) = ((x / win_x) * dim_x as f64, (y / win_y) * dim_y as f64);
                            mouse_position.set((x, y));
                        },
                        glutin::WindowEvent::MouseInput{ state, button, .. } => {
                            let key = match button {
                                glutin::MouseButton::Left => Some("mouse_left"),
                                glutin::MouseButton::Right => Some("mouse_right"),
                                glutin::MouseButton::Middle => Some("mouse_middle"),
                                _ => None,
                            };

                            key.map(|key| {
                                match state {
                                    glutin::ElementState::Pressed => {
                                        inputs.insert(key.to_string());
                                    },
                                    glutin::ElementState::Released => {
                                        inputs.remove(key);
                                    },
                                }
                            });
                        },
                        _ => (),
                    },
                    glutin::Event::DeviceEvent { event, .. } => match event {
                        glutin::DeviceEvent::Key(glutin::KeyboardInput { scancode, state, .. }) => {
                            let key = format!("key_{}", scancode);

                            match state {
                                glutin::ElementState::Pressed => {
                                    inputs.insert(key);
                                },
                                glutin::ElementState::Released => {
                                    inputs.remove(&key);
                                }
                            }
                        },
                        _ => (),
                    },
                    _ => (),
                }
            });
            let draw_commands = DrawPackage::new();

            let display_obj = Object::create_native_object(draw_commands.clone());
            let mouse = Object::create_native_object(Vector2(mouse_position.get().0 as f32, mouse_position.get().1 as f32));
            let inputs = inputs
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .into();

            callback.clone().call(ctx, vec![display_obj, mouse, inputs]);

            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);

            let mut commands = draw_commands.commands.borrow_mut();

            for cmd in commands.drain(..) {
                match cmd {
                    DrawCommand::ClearColor(v) => {
                        target.clear_color(v.0, v.1, v.2, 0.0);
                    },
                    DrawCommand::Line(start, end, color) => {
                        let points = vec![start, end];
                        let verts: Vec<_> = points
                            .into_iter()
                            .map(|v| Vertex { position: v, color: color })
                            .collect();
                        let vb = glium::VertexBuffer::new(&display, &verts).unwrap();
                        let uniforms = uniform! {
                            matrix: screen_scale
                        };
                        let _ = target.draw(&vb, &line_index_buffer, &solid_program, &uniforms, &line_params)
                            .unwrap_or_else(|err| eprintln!("Draw error: {}", err) );
                    },
                    DrawCommand::Lines(points, color) => {
                        let verts: Vec<_> = points
                            .into_iter()
                            .map(|v| Vertex { position: v, color: color })
                            .collect();
                        let vb = glium::VertexBuffer::new(&display, &verts).unwrap();
                        let uniforms = uniform! {
                            matrix: screen_scale
                        };
                        let _ = target.draw(&vb, &line_index_buffer, &solid_program, &uniforms, &line_params)
                            .unwrap_or_else(|err| eprintln!("Draw error: {}", err) );
                    },
                    DrawCommand::LineWidth(width) => {
                        line_params.line_width = Some(width);
                    },
                    DrawCommand::Circle(center, radius, color) => {
                        let verts: Vec<_> = circle_model
                            .iter()
                            .map(|v| Vertex { position: Vector2((v.0 * radius) + center.0, (v.1 * radius) + center.1), color: color })
                            .collect();
                        let vb = glium::VertexBuffer::new(&display, &verts).unwrap();
                        let uniforms = uniform! {
                            matrix: screen_scale
                        };
                        let _ = target.draw(&vb, &circle_index_buffer, &solid_program, &uniforms, &fill_params)
                            .unwrap_or_else(|err| eprintln!("Draw error: {}", err) );
                    },
                }
            }

            let _ = target.finish()
                .unwrap_or_else(|err| eprintln!("Finish error: {:?}", err) );
        }

        display.gl_window().hide();

        Object::Nil
    } else {
        Exception::message("Unable to initialize display").into()
    }
}
