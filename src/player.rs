use sfml::graphics::*;
use sfml::system::{Vector2i, Vector2f};
use sfml::window::Key;
use mousehandler::MouseMoveHandler;
use resources::Resources;
//use gamewindow::ScrollDir;

const SPEED: f32 = 10.;

pub struct Player<'s> {
    sprite: Sprite<'s>,
}

impl<'s> Player<'s> {
    pub fn new(res: &'s Resources) -> Player<'s> {
        let size = res.img.player.size();
        
        let mut sprite = Sprite::with_texture(&res.img.player);
        //sprite.set_position2f(size.x as f32 / -2., size.y as f32 / -2.);
        sprite.set_origin2f(size.x as f32 / 2., size.y as f32 / 2.);
        
        Player {
            sprite,
        }
    }
    
    /*
    pub fn handle_mouse() {
    }
    */
    
    pub fn update(&mut self, delta: f32) -> Option<Vector2f> {
        self.handle_keys_realtime(delta)
    }
    
    fn handle_keys_realtime(&mut self, delta: f32) -> Option<Vector2f> {
        if Key::W.is_pressed() {
            self.sprite.move2f(0., -SPEED);
        } else if Key::S.is_pressed() {
            self.sprite.move2f(0., SPEED);
        }
        
        if Key::A.is_pressed() {
            self.sprite.move2f(-SPEED, 0.);
        } else if Key::D.is_pressed() {
            self.sprite.move2f(SPEED, 0.);
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

impl<'s> MouseMoveHandler for Player<'s> {
    fn on_mouse_move(&mut self, target: &RenderTarget, mx: i32, my: i32) {
        let coords = target.map_pixel_to_coords_current_view(
                            &Vector2i::new(mx, my));
        
        // Player center
        let bounds = self.sprite.global_bounds();
        let cx = bounds.left + bounds.width / 2.;
        let cy = bounds.top + bounds.height / 2.;
        //println!("{} {}", cx, cy);
        
        // Distance from point to player
        let dx = coords.x - cx;
        let dy = coords.y - cy;
        //println!("{} {}", dx, dy);
        self.sprite.set_rotation(dy.atan2(dx).to_degrees() + 90.);
    }
}
