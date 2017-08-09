extern crate sfml;
extern crate survive;

use sfml::graphics::*;
use sfml::window::*;
use sfml::system::Clock;
use survive::*;

fn main() {
    let mut win = GameWindow::new();
    win.rwin.clear(&Color::white());
    win.rwin.display();
    
    let res = Resources::new();
    
    let mut player = Player::new(&res);
    
    let mut clock = Clock::start();
    'mainl: loop {
        let delta = clock.restart().as_seconds();
        
        win.rwin.clear(&Color::black());
        win.rwin.draw(&player);
        win.rwin.display();
        
        while let Some(e) = win.rwin.poll_event() {
            match e {
                Event::KeyPressed { code: Key::Escape, .. }
                        => break 'mainl,
                Event::Closed => break 'mainl,
                Event::MouseMoved { x, y } => {
                    player.on_mouse_move(&win.rwin, x, y);
                },
                _ => {},
            }
        }
        
        match player.update(delta) {
            Some(s) => win.scroll(s),
            _ => {}
        };
    }
}
