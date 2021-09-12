use macroquad::prelude::*;

pub const MINUTE_HAND_LENGTH: f32 = 200.0;
const MINUTE_HAND_THICKNESS: f32 = 2.5;

const HOUR_HAND_LENGTH: f32 = 100.0;
const HOUR_HAND_THICKNESS: f32 = 5.0;

fn draw_hand(deg: f32, length: f32, thickness: f32) {

	let x0: f32 = screen_width() / 2.0;
	let y0: f32 = screen_height() / 2.0;

	let angle = deg.to_radians();
	let x: f32 = length * angle.cos() + x0;
	let y: f32 = length * angle.sin() + y0;

	draw_line(x0, y0, x, y, thickness, WHITE);
}

// draws hour hand pointing to the given angle
pub fn draw_hour_hand(deg: f32) {
	draw_hand(deg, HOUR_HAND_LENGTH, HOUR_HAND_THICKNESS)
}

// draws hour hand pointing to the given angle
pub fn draw_minute_hand(deg: f32) {
	draw_hand(deg, MINUTE_HAND_LENGTH, MINUTE_HAND_THICKNESS)
}