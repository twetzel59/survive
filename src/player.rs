use sfml::graphics::*;
use sfml::system::{Vector2i, Vector2f};
use sfml::window::Key;
use mousehandler::MousePosHandler;
use resources::Resources;
//use gamewindow::ScrollDir;
use gamewindow::GameWindow;
use gamewindow::ScrollBounds;

const SPEED: f32 = 350.;
const SCROLL_BOUND: f32 = 25.;

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
    
    pub fn update(&mut self, delta: f32, win: &GameWindow) -> Option<Vector2f> {
        self.handle_keys_realtime(delta, &win.scroll_bounds(SCROLL_BOUND), &win.rwin)
    }
    
    fn handle_keys_realtime(&mut self, delta: f32, bounds: &ScrollBounds, target: &RenderTarget) -> Option<Vector2f> {
        //let rot = self.sprite.rotation().to_radians();
        
        if Key::W.is_pressed() {
            //self.sprite.move2f(rot.sin() * SPEED, rot.cos() * -SPEED);
            self.sprite.move2f(0., delta * -SPEED);
        } else if Key::S.is_pressed() {
            //self.sprite.move2f(rot.sin() * -SPEED, rot.cos() * SPEED);
            self.sprite.move2f(0., delta * SPEED);
        }
        
        if Key::A.is_pressed() {
            //self.sprite.move2f(rot.cos() * -SPEED, rot.sin() * -SPEED);
            self.sprite.move2f(delta * -SPEED, 0.);
        } else if Key::D.is_pressed() {
            //self.sprite.move2f(rot.cos() * SPEED, rot.sin() * SPEED);
            self.sprite.move2f(delta * SPEED, 0.);
        }
        
        let pixel = target.map_coords_to_pixel_current_view(&self.sprite.position());
        
        let mut rect = self.sprite.global_bounds();
        rect.left = pixel.x as f32;
        rect.top = pixel.y as f32;
        
        //println!("{:?}", rect);
        
        if bounds.left.intersection(&rect).is_some() {
            println!("scroll left");
        } else if bounds.right.intersection(&rect).is_some() {
            println!("scroll right");
        }
        
        if bounds.top.intersection(&rect).is_some() {
            println!("scroll up");
        } else if bounds.bottom.intersection(&rect).is_some() {
            println!("scroll down");
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

impl<'s> MousePosHandler for Player<'s> {
    fn mouse_pos(&mut self, target: &RenderTarget, mx: i32, my: i32) {
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
