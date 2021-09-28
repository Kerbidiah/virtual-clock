use macroquad::prelude::*;
mod draw_clock;
mod clock;

fn get_dt(sim_speed: f32) -> f32 {
	get_frame_time() * sim_speed
}

fn blend(lo: f32, hi: f32, percent: f32) -> f32 {
	let delta: f32 = hi - lo;
	
	lo + (delta * percent)
}

#[macroquad::main("Virtual Clock")]
async fn main() {
	let mut clock = clock::Clock{
		minute_hand: clock::Hand::new(1500. , 30.0),
		hour_hand: clock::Hand::new(1500., 15.0)
	};

	// simulation speed stuff
	let mut sim_speed: f32 = 1.0;
	let mut sim_speed_hold: f32 = sim_speed;
	let mut dt: f32;

	// turbo mode stuff
	let mut turbo_mode: bool = false;
	let mut reverse: bool = false;
	let og_hour_speed: f32 = clock.hour_hand.max_speed;
	let mut percent: f32 = 1.0;
	
	loop {
		if !turbo_mode {
			// adjust sim speed
			if is_key_pressed(KeyCode::Up) {
				sim_speed += 0.5;
			} else if is_key_pressed(KeyCode::Down) {
				sim_speed -= 0.5;
			} else if is_key_pressed(KeyCode::P) {
				sim_speed += 2.5;
			} else if is_key_pressed(KeyCode::O) {
				sim_speed -= 2.5;
			}
		} else {
			if is_key_pressed(KeyCode::Right) {
				percent += 0.05;
			} else if is_key_pressed(KeyCode::Left) {
				percent -= 0.05;
			}
			percent = percent.clamp(0.0, 1.0);
		}

		if is_key_pressed(KeyCode::F) && !turbo_mode { // enter turbo mode
			sim_speed_hold = sim_speed;
			turbo_mode = true;
			sim_speed = 1.0;

			clock.minute_hand.set_goal_speed(1.0);
			clock.hour_hand.set_goal_speed(1.0);
		} else if is_key_pressed(KeyCode::R) && turbo_mode { // toggle reverse in turbo mode
			if reverse {
				clock.minute_hand.set_goal_speed(1.0);
				clock.hour_hand.set_goal_speed(1.0);
			} else {
				clock.minute_hand.set_goal_speed(-1.0);
				clock.hour_hand.set_goal_speed(-1.0);
			}
			reverse = !reverse;
		} else if is_key_pressed(KeyCode::F) && turbo_mode { // exit turbo mode
			clock.minute_hand.cancel_goal();
			clock.hour_hand.cancel_goal();

			turbo_mode = false;
			reverse = false;
			sim_speed = sim_speed_hold;
		}

		if turbo_mode {
			clock.hour_hand.max_speed = blend(clock.minute_hand.max_speed / 12.0, og_hour_speed, percent);
		} else {
			clock.hour_hand.max_speed = og_hour_speed;
		}

		if sim_speed < 0.5 { // keep it above 0.5
			sim_speed = 0.5;
		}

		dt = get_dt(sim_speed); // delta t

		clear_background(BLACK); // reset screen to black
		draw_text( // fps counter
			&format!("FPS: {:.2}", get_fps()),
			20.0, 20.0, 20.0, YELLOW
		);
		
		// clock stuff
		clock.minute_hand.advance(6.0 / 60.0, dt);
		clock.hour_hand.advance(0.5 / 60.0, dt);
		clock.minute_hand.execute_goal(dt); // does nothing if there is no goal
		clock.hour_hand.execute_goal(dt);

		clock.draw();
		
		if !turbo_mode {
			draw_text( // simulation speed
				&format!("{:.1}x speed", sim_speed),
				20.0, 40.0, 20.0, YELLOW
			);
		} else {
			draw_text( // turbo mode
				"TURBO MODE!!!",
				20.0, 40.0, 20.0, RED
			);
			draw_text( // hour max speed
				&format!("{} RPS", clock.hour_hand.max_speed / 360.0),
				20.0, 80.0, 20.0, YELLOW
			);
			draw_text( // bend percent
				&format!("{:.0}% blend", percent * 100.0),
				20.0, 60.0, 20.0, YELLOW
			);
		}

		next_frame().await // aims for 60 fps it seems
	}
}