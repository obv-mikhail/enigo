extern crate enigo;
use enigo::{Enigo, MouseControllable, MouseButton};
use std::{thread, time};

fn main() {
    let wait_time = time::Duration::from_millis(200);
    let mut enigo = Enigo::new();

    thread::sleep(wait_time);

    enigo.mouse_move_to(500, 200);
    thread::sleep(wait_time);

    enigo.mouse_down(MouseButton::Left);
    thread::sleep(wait_time);

    enigo.mouse_move_relative(100, 100);
    thread::sleep(wait_time);

    enigo.mouse_up(MouseButton::Left);
    thread::sleep(wait_time);

    enigo.mouse_click(MouseButton::Left);
    thread::sleep(wait_time);

    enigo.mouse_scroll_x(2);
    thread::sleep(wait_time);

    enigo.mouse_scroll_x(-2);
    thread::sleep(wait_time);

    enigo.mouse_scroll_y(2);
    thread::sleep(wait_time);

    enigo.mouse_scroll_y(-2);
    thread::sleep(wait_time);
}
