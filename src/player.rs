use sfml::graphics::*;
use sfml::system::{Vector2i, Vector2f};
use sfml::window::Key;
use mousehandler::MousePosHandler;
use registry::terrain::Terrain;
use resources::Resources;
//use gamewindow::ScrollDir;
use gamewindow::GameWindow;
use gamewindow::ScrollBounds;

const SPEED: f32 = 350.;
const SCROLL_BOUND: f32 = 64.;

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
        let res = self.handle_keys_realtime(delta,
                                            &win.scroll_bounds(SCROLL_BOUND),
                                            &win.rwin);
        
        res
    }
    
    pub fn is_in_water(&self, world: &[Terrain]) -> bool {
        use tiles::TILE_SCALE;
        use worldgen::WORLD_SIZE;
        
        /*
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
            }
        }
        */
        
        let pos = self.sprite.position();        
        let pos = Vector2i::new(((pos.x / TILE_SCALE) + (WORLD_SIZE as f32 / 2.)) as i32,
                                ((pos.y / TILE_SCALE) + (WORLD_SIZE as f32 / 2.)) as i32);
        //println!("{:?}", pos);
        
        if pos.x < 0 || pos.y < 0 || pos.x > WORLD_SIZE as i32 || pos.y > WORLD_SIZE as i32 {
            return false;
        }
        
        //println!("{:?}: {:?}", pos, world[(pos.x as usize) * WORLD_SIZE as usize + pos.y as usize]);
        
        match world[(pos.x as usize) * WORLD_SIZE as usize + pos.y as usize] {
            Terrain::Water => true,
            _ => false,
        }
    }
    
    fn handle_keys_realtime(&mut self, delta: f32, bounds: &ScrollBounds, target: &RenderTarget) -> Option<Vector2f> {
        //let pixel = target.map_coords_to_pixel_current_view(&pos);
        
        let mut rect = self.sprite.global_bounds();
        let coords = target.map_coords_to_pixel_current_view(
                            &Vector2f::new(rect.left, rect.top));
        rect.left = coords.x as f32;
        rect.top = coords.y as f32;
        
        //println!("{:?}", rect);
        
        let mut scroll_left = false;
        let mut scroll_right = false;
        let mut scroll_up = false;
        let mut scroll_down = false;
        
        if bounds.left.intersection(&rect).is_some() {
            //println!("scroll left");
            scroll_left = true;
        } else if bounds.right.intersection(&rect).is_some() {
            //println!("scroll right");
            scroll_right = true;
        }
        
        if bounds.top.intersection(&rect).is_some() {
            //println!("scroll up");
            scroll_up = true;
        } else if bounds.bottom.intersection(&rect).is_some() {
            //println!("scroll down");
            scroll_down = true;
        }
        
        //
        
        let rot = self.sprite.rotation().to_radians();
        let mut change = Vector2f::new(0., 0.);
        
        if Key::W.is_pressed() {
            //self.sprite.move2f(rot.sin() * delta * SPEED, rot.cos() * delta * -SPEED);
            change.x += rot.sin() * delta * SPEED;
            change.y += rot.cos() * delta * -SPEED;
            
            //self.sprite.move2f(0., delta * -SPEED);
        } else if Key::S.is_pressed() {
            //self.sprite.move2f(rot.sin() * delta * -SPEED, rot.cos() * delta * SPEED);
            change.x += rot.sin() * delta * -SPEED;
            change.y += rot.cos() * delta * SPEED;
            
            //self.sprite.move2f(0., delta * SPEED);
        }
        
        if Key::A.is_pressed() {
            //self.sprite.move2f(rot.cos() * delta * -SPEED, rot.sin() * delta * -SPEED);
            change.x += rot.cos() * delta * -SPEED;
            change.y += rot.sin() * delta * -SPEED;
            
            //self.sprite.move2f(delta * -SPEED, 0.);
        } else if Key::D.is_pressed() {
            //self.sprite.move2f(rot.cos() * delta * SPEED, rot.sin() * delta * SPEED);
            change.x += rot.cos() * delta * SPEED;
            change.y += rot.sin() * delta * SPEED;
            
            //self.sprite.move2f(delta * SPEED, 0.);
        }
        
        self.sprite.move_(&change);
        
        let mut scroll = Vector2f::new(0., 0.);
        if (scroll_left && change.x < 0.) || (scroll_right && change.x > 0.) {
            scroll.x = change.x;
        }
        if (scroll_up && change.y < 0.) || (scroll_down && change.y > 0.) {
            scroll.y = change.y;
        }
        
        if scroll.x != 0. || scroll.y != 0. {
            Some(scroll)
        } else {
            None
        }
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
        
        // Set the rotation angle based on distances
        self.sprite.set_rotation(dy.atan2(dx).to_degrees() + 90.);
    }
}
