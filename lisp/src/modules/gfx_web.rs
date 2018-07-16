use wasm_bindgen::prelude::*;

use std::collections::HashSet;

use super::*;
use super::gfx::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<Vector3<f32>> for Color {
    fn from(other: Vector3<f32>) -> Self {
        Color {
            r: other.0,
            g: other.1,
            b: other.2,
        }
    }
}

impl From<Vector2<f32>> for Point {
    fn from(other: Vector2<f32>) -> Self {
        Point {
            x: other.0,
            y: other.1,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = massLispGfx, js_name = newWindow)]
    pub fn new_window(width: u32, height: u32) -> u32;
    #[wasm_bindgen(js_namespace = massLispGfx, js_name = pollEvents)]
    pub fn poll_events(window: u32) -> Vec<u32>;
    #[wasm_bindgen(js_namespace = massLispGfx, js_name = pollMouse)]
    pub fn poll_mouse(window: u32) -> Vec<f32>;
    #[wasm_bindgen(js_namespace = massLispGfx, js_name = setClearColor)]
    pub fn set_clear_color(window: u32, color: Color);
    #[wasm_bindgen(js_namespace = massLispGfx, js_name = setLineWidth)]
    pub fn set_line_width(window: u32, width: f32);
    #[wasm_bindgen(js_namespace = massLispGfx, js_name = drawLine)]
    pub fn draw_line(window: u32, start: Point, end: Point, color: Color);
    #[wasm_bindgen(js_namespace = massLispGfx, js_name = drawLineList)]
    pub fn draw_line_list(window: u32, x_coords: &[f32], y_coords: &[f32], color: Color);
    #[wasm_bindgen(js_namespace = massLispGfx, js_name = drawCircle)]
    pub fn draw_circle(window: u32, point: Point, radius: f32, color: Color);
    #[wasm_bindgen(js_namespace = massLispGfx, js_name = draw)]
    pub fn draw(window: u32);
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

    let window = new_window(dim_x, dim_y);
    let args = vec![(window as i64).into(), Vec::<Object>::new().into(), callback];
    let callback = NativeFunction {
        ptr: do_frame,
    };

    ctx.do_yield(callback.into(), args);

    Object::Nil
}


const MOUSE: u32 =   0x80000000;
const KEY: u32   =   0x40000000;
const PRESSED: u32 = 0x20000000;

fn do_frame(ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();
    let window = args.next()
        .map(|a| integer_impl(&a))
        .and_then(|a| match a {
            Number::Integer(num) => Some(num as u32),
            _ => None
        })
        .unwrap_or(0);
    let last_inputs = args
        .next()
        .and_then(|i| match i {
            Object::List(l) => {
                Some(l.iter().map(|s| string_impl(s)).collect::<Vec<String>>())
            },
            _ => None,
        }).unwrap_or(Vec::new());
    let callback = args.next().unwrap_or(Object::Nil);

    let mut closed = false;

    let mut inputs: HashSet<String> = HashSet::new();
    last_inputs
        .into_iter()
        .for_each(|i| {
            inputs.insert(i);
        });

    let events = poll_events(window);
    events.into_iter().for_each(|event| {
        if event & MOUSE != 0 {
            if event & PRESSED != 0 {
                inputs.insert("mouse_left".to_string());
            } else {
                inputs.remove("mouse_left");
            }
        } else if event & KEY != 0 {
            let key = format!("key_{}", event & 0xFFFF);
            if key == "key_81" { closed = true; }
            if event & PRESSED != 0 {
                inputs.insert(key);
            } else {
                inputs.remove(&key);
            }
        }
    });

    let mouse_position = poll_mouse(window);

    let draw_commands = DrawPackage::new();

    let display_obj = Object::create_native_object(draw_commands.clone());
    let mouse = Object::create_native_object(Vector2(*mouse_position.get(0).unwrap_or(&0.0), *mouse_position.get(1).unwrap_or(&0.0)));

    let callback_inputs: Object = inputs
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .into();

    callback.clone().call(ctx, vec![display_obj, mouse, callback_inputs.clone()]);

    let mut commands = draw_commands.commands.borrow_mut();

    for cmd in commands.drain(..) {
        match cmd {
            DrawCommand::ClearColor(v) => {
                set_clear_color(window, v.into());
            },
            DrawCommand::Line(start, end, color) => {
                draw_line(window, start.into(), end.into(), color.into());
            },
            DrawCommand::Lines(points, color) => {
                let (x, y): (Vec<_>, Vec<_>) = points
                    .iter()
                    .map(|p| (p.0, p.1))
                    .unzip();
                draw_line_list(window, &x, &y, color.into());
            },
            DrawCommand::LineWidth(width) => {
                set_line_width(window, width);
            },
            DrawCommand::Circle(center, radius, color) => {
                draw_circle(window, center.into(), radius, color.into());
            },
        }
    }

    draw(window);

    if !closed {
        let args = vec![(window as i64).into(), callback_inputs, callback];
        let callback = NativeFunction {
            ptr: do_frame,
        };

        ctx.do_yield(callback.into(), args);
    }

    Object::Nil
}
