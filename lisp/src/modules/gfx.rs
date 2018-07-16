use std::rc::Rc;
use std::cell::RefCell;

use super::*;

pub struct Gfx;

impl NativeModule for Gfx {
    fn register(&mut self) -> NativeModuleDescription {
        let mut desc = NativeModuleDescription::new("gfx");

        desc.register_function("create_window", create_window);
        desc.register_function("set_clear_color", set_clear_color);
        desc.register_function("set_line_width", set_line_width);
        desc.register_function("draw_line", draw_line);
        desc.register_function("draw_circle", draw_circle);
        desc.register_function("draw_line_list", draw_line_list);

        desc.register_function("vec2", vec2);
        desc.register_function("vec3", vec3);

        desc.register_function("vec_x", vec_x);
        desc.register_function("vec_y", vec_y);
        desc.register_function("vec_z", vec_z);
        desc
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2<T>(pub T, pub T);
#[derive(Debug, Clone, Copy)]
pub struct Vector3<T>(pub T, pub T, pub T);

impl<T: Copy> Vector3<T> {
    pub fn as_array(&self) -> [T; 3] {
        [self.0, self.1, self.2]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vector2<f32>,
    pub color: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub enum DrawCommand {
    ClearColor(Vector3<f32>),
    Line(Vector2<f32>, Vector2<f32>, Vector3<f32>),
    Lines(Vec<Vector2<f32>>, Vector3<f32>),
    LineWidth(f32),
    Circle(Vector2<f32>, f32, Vector3<f32>),
}

#[derive(Debug, Clone)]
pub struct DrawPackage {
    pub commands: Rc<RefCell<Vec<DrawCommand>>>,
}

impl DrawPackage {
    pub fn new() -> Self {
        Self { commands: Rc::new(RefCell::new(Vec::new())) }
    }

    pub fn push(&self, cmd: DrawCommand) {
        let mut commands =  self.commands.borrow_mut();
        (*commands).push(cmd)
    }
}

pub fn create_window(ctx: &mut Context, args: Vec<Object>) -> Object {
    super::gfx_platform::create_window(ctx, args)
}

pub fn set_clear_color(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let draw_commands: Option<DrawPackage> = to_native(args.get(0));
    let color: Option<Vector3<f32>> = to_native(args.get(1));
    if let (Some(draw_commands), Some(color)) = (draw_commands, color) {
        draw_commands.push(DrawCommand::ClearColor(color));

        Object::create_native_object(draw_commands)
    } else {
        Object::Nil
    }
}

pub fn set_line_width(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let draw_commands: Option<DrawPackage> = to_native(args.get(0));
    let width = args.get(1)
        .map(|a| core::float_impl(a))
        .and_then(|i| if let Number::Float(i) = i { Some(i as f32) } else { None } );

    if let (Some(draw_commands), Some(width)) = (draw_commands, width) {
        draw_commands.push(DrawCommand::LineWidth(width));

        Object::create_native_object(draw_commands)
    } else {
        Object::Nil
    }
}

pub fn draw_line(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let draw_commands: Option<DrawPackage> = to_native(args.get(0));
    let start: Option<Vector2<f32>> = to_native(args.get(1));
    let end: Option<Vector2<f32>> = to_native(args.get(2));
    let color: Option<Vector3<f32>> = to_native(args.get(3));
    if let (Some(draw_commands), Some(start), Some(end), Some(color)) = (draw_commands, start, end, color) {
        draw_commands.push(DrawCommand::Line(start, end, color));

        Object::create_native_object(draw_commands)
    } else {
        Object::Nil
    }
}
pub fn draw_circle(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let draw_commands: Option<DrawPackage> = to_native(args.get(0));
    let center: Option<Vector2<f32>> = to_native(args.get(1));
    let radius = to_float(args.get(2));
    let color: Option<Vector3<f32>> = to_native(args.get(3));
    if let (Some(draw_commands), Some(center), Some(color)) = (draw_commands, center, color) {
        draw_commands.push(DrawCommand::Circle(center, radius, color));

        Object::create_native_object(draw_commands)
    } else {
        Object::Nil
    }
}

pub fn draw_line_list(_ctx: &mut Context, args: Vec<Object>) -> Object {
    match args.get(1) {
        Some(Object::List(l)) => {
            let points: Vec<Vector2<f32>> = l.iter().filter_map(|n| to_native(Some(n))).collect();
            let draw_commands: Option<DrawPackage> = to_native(args.get(0));
            let color: Option<Vector3<f32>> = to_native(args.get(2));

            if points.len() < 2 { Object::Nil } else {
                if let (Some(draw_commands), Some(color)) = (draw_commands, color) {
                    draw_commands.push(DrawCommand::Lines(points, color));
                    Object::create_native_object(draw_commands)
                } else {
                    Object::Nil
                }
            }
        },
        _ => Object::Nil
    }
}

pub fn vec2(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let x = to_float(args.get(0));
    let y = to_float(args.get(1));

    Object::create_native_object(Vector2(x, y))
}

pub fn vec3(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let x = to_float(args.get(0));
    let y = to_float(args.get(1));
    let z = to_float(args.get(2));

    Object::create_native_object(Vector3(x, y, z))
}

pub fn vec_x(_ctx: &mut Context, args: Vec<Object>) -> Object {
    if let Some(vec) = to_native::<Vector2<f32>>(args.get(0)) {
        (vec.0 as f64).into()
    } else if let Some(vec) = to_native::<Vector3<f32>>(args.get(0)) {
        (vec.0 as f64).into()
    } else {
        Object::Nil
    }
}

pub fn vec_y(_ctx: &mut Context, args: Vec<Object>) -> Object {
    if let Some(vec) = to_native::<Vector2<f32>>(args.get(0)) {
        (vec.1 as f64).into()
    } else if let Some(vec) = to_native::<Vector3<f32>>(args.get(0)) {
        (vec.1 as f64).into()
    } else {
        Object::Nil
    }
}

pub fn vec_z(_ctx: &mut Context, args: Vec<Object>) -> Object {
    if let Some(vec) = to_native::<Vector3<f32>>(args.get(0)) {
        (vec.2 as f64).into()
    } else {
        Object::Nil
    }
}

fn to_native<T: Clone + 'static>(obj: Option<&Object>) -> Option<T> {
    if let Some(Object::NativeObject(o)) = obj {
        o.downcast_ref::<T>().map(|o| o.clone())
    } else {
        None
    }
}

fn to_float<T: ::std::borrow::Borrow<Object>>(arg: Option<T>) -> f32 {
    if let Some(Number::Float(f)) = arg.map(|a| core::float_impl(a.borrow())) {
        f as f32
    } else {
        0.0
    }
}
