extern crate sfml;
extern crate survive;

use sfml::graphics::*;
use sfml::window::*;
use survive::*;

fn main() {
    let mut win = GameWindow::new();
    
    'mainl: loop {
        win.rwin.clear(&Color::black());
        win.rwin.display();
        
        while let Some(e) = win.rwin.poll_event() {
            match e {
                Event::KeyPressed { code: Key::Escape, .. }
                        => break 'mainl,
                Event::Closed => break 'mainl,
                _ => {},
            }
        }
    }
}
