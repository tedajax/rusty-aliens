extern crate sdl2;

use sdl2::pixels::Color;

mod player;
mod gametime;

fn main() {
    let mut sdl_ctx = sdl2::init().video().unwrap();
    
    let window = sdl_ctx.window("Rusty Aliens", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
        
    let mut renderer = window.renderer().present_vsync().build().unwrap();
    let mut drawer = renderer.drawer();
    
    let mut running = true;
    let mut gametime = gametime::GameTime::new();
    
    let mut player = player::Player::new();
    
    while running {
        for event in sdl_ctx.event_pump().poll_iter() {
            use sdl2::event::Event;
            use sdl2::keycode::KeyCode;
            
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: KeyCode::Escape, ..} => {
                    running = false
                },
                _ => {}
            }
        }
               
        gametime.update();
               
        // Update
        player.update(gametime.dt());
        
        // Render
        drawer.set_draw_color(Color::RGB(0, 0, 0));
        drawer.clear();
        
        player.render(&mut drawer);
        
        drawer.present();
    }   
}