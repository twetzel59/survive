use core::slice;
use sfml::graphics::*;
use sfml::system::Vector2f;
use resources::Resources;

const TILE_SCALE: f32 = 128.;

pub struct Tile<'s> {
    sprite: Sprite<'s>
}

impl<'s> Drawable for Tile<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(
                                   &'se self, 
                                   target: &mut RenderTarget, 
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh
    {
        target.draw_sprite(&self.sprite, states);
    }
}

pub struct TileManager<'s> {
    tiles: Vec<Tile<'s>>,
}

impl<'s> TileManager<'s> {
    pub fn new(res: &'s Resources) -> TileManager {
        let mut tiles = Vec::new();
        
        let size = res.img.nyan.size();
        let size = Vector2f::new(size.x as f32, size.y as f32);
        let origin = Vector2f::new(size.x / 2., size.y / 2.);
        
        for x in -4..4 {
            for y in -4..4 {
                let mut sprite = Sprite::with_texture(&res.img.nyan);
                sprite.set_origin(&origin);
                sprite.set_scale2f(TILE_SCALE, TILE_SCALE);
                sprite.set_position2f(TILE_SCALE * size.x * x as f32,
                                      TILE_SCALE * size.y * y as f32);

                /*
                let cr = ((x + 5) as u8) * 25;
                let cg = ((y + 5) as u8) * 25;
                sprite.set_color(&Color::rgb(cr, cg, 255));
                
                tiles.push(Tile { sprite });
                */
            }
        }
        
        TileManager {
            tiles,
        }
    }
}

impl<'a, 's> IntoIterator for &'a TileManager<'s> {
    type Item = &'a Tile<'s>;
    type IntoIter = slice::Iter<'a, Tile<'s>>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.tiles.iter()
    }
}
