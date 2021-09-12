use macroquad::prelude::*;
use crate::draw_clock;

#[derive(Debug)]
#[allow(dead_code)] // TODO: remove
enum Goal {
	Positon(f32), // go to this angle
	Direction(f32), // point in equivilent direction
	Speed(f32) // spin at a certian percent of max speed
}

#[derive(Debug)]
pub struct Hand {
	pub angle: f32, // in degrees
	pub max_speed: f32, // in degrees per second
	goal: Option<Goal>
}

impl Hand {
	pub fn new(rpm: f32, gear_ratio: f32) -> Hand {
		Hand {
			angle: -90.0, // start pointing up
			max_speed: rpm * 6.0 / gear_ratio,
			goal: None
		}
	}

	#[inline] #[allow(dead_code)]
	fn equiv_angle(&mut self) { // not sure if doing this is helpfull
		self.angle %= 360.0;
	}

	#[inline]
	fn validate_speed(&self, dps: f32, dt: f32) -> Result<(), f32> {
		let requested_speed = dps.abs() * dt;

		if requested_speed <= self.max_speed {
			Ok(())
		} else {
			Err(requested_speed / self.max_speed)
		}
	}

	#[inline]
	fn max_move(&self, dt: f32) -> f32 {
		self.max_speed * dt
	}

	pub fn advance(&mut self, dps: f32, dt: f32) {
		if self.goal.is_none() {

			let check = self.validate_speed(dps, dt);

			if check.is_ok() {
				self.angle += dps * dt;
			} else {
				dbg!((
					&self,
					check.unwrap_err()
				));

				// TODO: figure out what to do when you try to go too fast
				self.angle += dps * dt;
			}
		}
	}

	pub fn set_goal_position(&mut self, degrees: f32) {
		self.goal = Some(Goal::Positon(degrees));
	}

	#[allow(dead_code)]
	pub fn set_goal_angle(&mut self, angle: f32) {
		self.goal = Some(Goal::Direction(angle));
		todo!();
	}

	#[allow(dead_code)]
	pub fn set_goal_relative(&mut self, delta: f32) {
		self.set_goal_position(self.angle + delta);
	}

	pub fn set_goal_speed(&mut self, percent: f32) {
		self.goal = Some(Goal::Speed(percent));
	}

	pub fn cancel_goal(&mut self) {
		self.goal = None;
	}

	#[allow(dead_code)]
	pub fn execute_goal(&mut self, dt: f32) {
		match self.goal {
			None => {}, // do nothing
			Some(Goal::Positon(destination)) => {
				let delta = destination - self.angle;
				let theta = self.max_move(dt);

				if delta.abs() <= theta { // if destination is in range
					self.angle = destination; // move to destination
					self.cancel_goal(); // remove the goal
				} else if delta > 0.0 {
					self.angle += theta;
				} else {
					self.angle -= theta;
				}
			},
			Some(Goal::Direction(angle)) => { // TODO
				dbg!(angle); // gets cargo check to shutup about variable not used
				todo!();
			},
			Some(Goal::Speed(percent)) => {
				self.angle += percent * self.max_move(dt);
			}
		}
	}


}

#[derive(Debug)]
pub struct Clock {
	pub minute_hand: Hand,
	pub hour_hand: Hand
}

impl Clock {
	pub fn draw(&self) {
		draw_clock::draw_minute_hand(self.minute_hand.angle);
		draw_clock::draw_hour_hand(self.hour_hand.angle);
	}
}