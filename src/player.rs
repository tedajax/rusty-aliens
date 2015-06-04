use sdl2::render::{RenderDrawer};
use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub struct Player {
	pos_x: f32,
	width: i32,
}

impl Player {
	pub fn new() -> Player {
		Player { pos_x: 0_f32, width: 40 }
	}
	
	pub fn update(&mut self, dt: f32) {
		self.pos_x += 160_f32 * dt;
	}
	
	pub fn render(&self, drawer: &mut RenderDrawer) {
		drawer.set_draw_color(Color::RGB(0, 255, 0));
		
		let r = Rect {
			x: self.pos_x as i32 - self.width / 2,
			y: 460,
			w: self.width,
			h: 20
		};
		
		drawer.fill_rect(r);
	}
}