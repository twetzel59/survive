use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::Key;
use resources::Resources;
//use gamewindow::ScrollDir;

pub struct Player<'s> {
    sprite: Sprite<'s>,
}

impl<'s> Player<'s> {
    pub fn new(res: &'s Resources) -> Player<'s> {
        Player {
            sprite: Sprite::with_texture(&res.img.player),
        }
    }
    
    pub fn handle_mouse() {
    }
    
    pub fn handle_keys_realtime(&mut self, delta: f32) -> Option<Vector2f> {
        if Key::W.is_pressed() {
        } else if Key::S.is_pressed() {
        }
        
        if Key::A.is_pressed() {
        } else if Key::D.is_pressed() {
        }
        
        None
    }
}

impl<'s> Drawable for Player<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(
                                   &'se self, 
                                   target: &mut RenderTarget, 
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh
    {
        target.draw_sprite(&self.sprite, states);
    }
}
