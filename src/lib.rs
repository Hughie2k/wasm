use console_error_panic_hook;
use othello::board::{self, Board, BoardState};
use othello::evaluation::{best_move, better_eval};
use std::cell::{Cell, RefCell};
use std::f64;
use std::rc::Rc;
use std::thread;
//use wasm_thread as thread;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, CanvasRenderingContext2d};
use std::sync::{mpsc, self};


// The code probably would work if only I could use thread::spawn in wasm, but of course, I cannot,
// which makes perfect sense and does not make me annoyed in the slightest, I mean, I just
// implemented it in a way which doesn't work for the fun of it; it wasn't a waste of time at all
// (I'M SO FUCKING ANGRY).
#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    web_sys::console::log_1(&JsValue::from_str("hi"));
    // This is a test to see if threads work
    {
        thread::spawn(|| console::log_1(&JsValue::from_str("Hell worldo")));
    }
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

    let mut board = Board::default();
    let (tx, rx) = mpsc::channel::<u64>();
    let context = sync::Arc::new(sync::Mutex::new(context));
    let piece_radius = 30.0;
    let sqsize = (canvas.width().min(canvas.height()) as f64) / 8.0;
    web_sys::console::log_1(&JsValue::from_str(format!("canvas width = {}, canvas height = {}, sqsize = {}", canvas.width(), canvas.height(), sqsize).as_str()));
    draw_board(&context.lock().unwrap(), sqsize, piece_radius, &board);
    let closure = Closure::wrap(Box::new(move || {
        if let Ok(move_bit) = rx.try_recv() {
            // We *must* verify that this move is correct elsewhere
            board.make_move(move_bit);
            draw_board(&context.lock().expect("Poisoned"), sqsize, piece_radius, &board);
        }
    }) as Box<dyn FnMut()>);
    web_sys::window().unwrap().request_animation_frame(closure.as_ref().unchecked_ref());
    closure.forget();
    let mut our_board = Board::default();
    console::log_1(&JsValue::from_str(format!("{sqsize}").as_str()));
    let document = Rc::new(RefCell::new(document));
    let human_is_black = true;
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        // let mut our_board = board.lock().expect("Poisoned Mutex? I have no idea what I'm doing anyway :(");
        
        if our_board.black_moving == human_is_black && our_board.board_state == BoardState::Ongoing {
            let (x, y) = (event.offset_x(), event.offset_y());
            let (move_x, move_y) = (x / sqsize as i32, y / sqsize as i32);
            console::log_1(&JsValue::from_str(
                format!("move_x = {move_x}, move_y = {move_y}").as_str(),
            ));
            console::log_1(&JsValue::from_str(
                format!("(move_x * 8 + move_y) = {}", move_x * 8 + move_y).as_str(),
            ));
            console::log_1(&JsValue::from_str("hello?"));
            let move_bit: u64 = 1u64 << (move_x * 8 + move_y);
            if let Ok(_) = our_board.clone().safe_make_move(move_bit) {
                tx.send(move_bit).unwrap();
            }
        };
        // Threads because I don't want to block this callback otherwise the board will take
        // aaaages to draw :(
        let mut ai_board = our_board.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            while ai_board.black_moving != human_is_black && ai_board.board_state == BoardState::Ongoing {
                let move_bit = best_move(better_eval, &ai_board, 6);
                tx.send(move_bit).unwrap();
                ai_board.make_move(move_bit);
            }
        });
        

        let (waiting, to_move) = match our_board.black_moving {
            true => ("white", "black"),
            false => ("black", "white"),
        };
        document
            .borrow()
            .get_element_by_id(waiting)
            .unwrap()
            .set_inner_html(
                format!("{waiting}: {}", our_board.waiting.clone().count()).as_str(),
            );
        document
            .borrow()
            .get_element_by_id(to_move)
            .unwrap()
            .set_inner_html(
                format!("{to_move}: {}", our_board.to_move.clone().count()).as_str(),
            );

        match our_board.board_state {
            BoardState::Ongoing => (),
            BoardState::Won => document
                .borrow()
                .get_element_by_id("state")
                .unwrap()
                .set_inner_html(
                    (String::from(match our_board.black_moving {
                        true => "Black",
                        false => "White",
                    }) + " wins!")
                        .as_str(),
                ),
            BoardState::Drawn => document
                .borrow()
                .get_element_by_id("state")
                .unwrap()
                .set_inner_html("The game is drawn!"),
        };
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
    closure.forget();
}

fn draw_board(ctx: &CanvasRenderingContext2d, square_size: f64, piece_radius: f64, board: &Board) {
    let (black, white) = match board.black_moving {
        true => (board.to_move.bits, board.waiting.bits),
        false => (board.waiting.bits, board.to_move.bits),
    };
    for i in 0..8 {
        for j in 0..8 {
            let (i, j) = (i as f64, j as f64);
            ctx.begin_path();
            ctx.set_fill_style(&JsValue::from_str("seagreen"));
            ctx.rect(i * square_size, j * square_size, square_size, square_size);
            ctx.fill();
            ctx.stroke();
            ctx.move_to(
                (i + 0.5) * square_size + piece_radius,
                (j + 0.5) * square_size,
            );
            let mut moves = board.each_move();
            let (bbit, wbit) = (
                black >> (8 * i as u8 + j as u8) & 1,
                white >> (8 * i as u8 + j as u8) & 1,
            );
            let is_move = moves.any(|bit| bit == (1 << (8 * i as u8 + j as u8)));
            if (bbit | wbit) == 1 || is_move {
                ctx.begin_path();
                if bbit == 1 {
                    ctx.set_fill_style(&JsValue::from_str("black"));
                } else if wbit == 1 {
                    ctx.set_fill_style(&JsValue::from_str("white"));
                } else if is_move {
                    ctx.set_fill_style(&JsValue::from_str("mediumspringgreen"));
                }
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
}
