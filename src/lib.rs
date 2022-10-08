use othello::board::Board;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let sqsize = 80;
    let piece_radius = 30;
    for i in 0..8 {
        for j in 0..8 {
            context.begin_path();
            context.set_fill_style(&JsValue::from_str("green"));
            context.rect(
                (i * sqsize) as f64,
                (j * sqsize) as f64,
                sqsize as f64,
                sqsize as f64,
            );
            context.fill();
            context.stroke();
            context.move_to(
                (i as f64 + 0.5) * sqsize as f64 + piece_radius as f64,
                (j as f64 + 0.5) * sqsize as f64,
            );
            context.begin_path();
            context.set_fill_style(&JsValue::from_str("black"));
            context.arc(
                (i as f64 + 0.5) * sqsize as f64,
                (j as f64 + 0.5) * sqsize as f64,
                piece_radius as f64,
                0.0,
                2.0 * f64::consts::PI,
            );
            context.fill();
        }
    }
}

fn draw_board(ctx: &CanvasRenderingContext2d, square_size: f64, piece_radius: f64) {
    for i in 0..8 {
        for j in 0..8 {
            let (i, j) = (i as f64, j as f64);
            ctx.begin_path();
            ctx.set_fill_style(&JsValue::from_str("green"));
            ctx.rect(i * square_size, j * square_size, square_size, square_size);
            ctx.fill();
            ctx.stroke();
            ctx.move_to(
                (i + 0.5) * square_size + piece_radius,
                (j + 0.5) * square_size,
            );
            ctx.begin_path();
            ctx.set_fill_style(&JsValue::from_str("black"));
            ctx.arc(
                (i + 0.5) * square_size,
                (j + 0.5) * square_size,
                piece_radius,
                0.0,
                2.0 * f64::consts::PI,
            );
            ctx.fill();
        }
    }
}
