extern crate sfml;
extern crate survive;

use sfml::graphics::*;
use sfml::window::*;
use sfml::system::{Clock, Vector2i};
use survive::*;

fn main() {
    let mut win = GameWindow::new();
    win.rwin.clear(&Color::white());
    win.rwin.display();
    
    let res = Resources::new();
    
    let mut wg = WorldGen::new();
    //let test = Sprite::with_texture(&wg.textures[9]);
    
    let tilemgr = TileManager::new(wg.textures());
    
    let mut player = Player::new(&res);
    
    let mut clock = Clock::start();
    'mainl: loop {
        let delta = clock.restart().as_seconds();
        
        win.rwin.clear(&Color::black());
        for i in &tilemgr {
            win.rwin.draw(i);
        }
        //win.rwin.draw(&test);
        win.rwin.draw(&player);
        win.rwin.display();
        
        while let Some(e) = win.rwin.poll_event() {
            match e {
                Event::KeyPressed { code: Key::Escape, .. }
                        => break 'mainl,
                Event::Closed => break 'mainl,
                Event::Resized { width, height } => win.on_resize(width, height),
                _ => {},
            }
        }
        
        let Vector2i { x: mx, y: my } = win.rwin.mouse_position();
        player.mouse_pos(&win.rwin, mx, my);
        
        match player.update(delta, &win) {
            Some(s) => win.scroll(&s),
            None => {},
        };
    }
}
