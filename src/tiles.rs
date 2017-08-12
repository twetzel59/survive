use core::slice;
use sfml::graphics::*;
use sfml::system::Vector2f;
use resources::Resources;

pub const TILE_SCALE: f32 = 4.;
pub const TILES_ROW: i32 = 4;
const TILES_ROW_HALF: i32 = TILES_ROW / 2;

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
    pub fn new(tex: &'s [Texture]) -> TileManager {
        let mut tiles = Vec::new();
        
        let size = tex[0].size();
        let size = Vector2f::new(size.x as f32, size.y as f32);
        let origin = Vector2f::new(size.x / 2., size.y / 2.);
        
        for x in -TILES_ROW_HALF..TILES_ROW_HALF {
            for y in -TILES_ROW_HALF..TILES_ROW_HALF {
                println!("index: {}", ((x + TILES_ROW_HALF) * TILES_ROW + y + TILES_ROW_HALF) as usize);
                let mut sprite = Sprite::with_texture(&tex[((x + TILES_ROW_HALF) * TILES_ROW + y + TILES_ROW_HALF) as usize]);
                sprite.set_origin(&origin);
                sprite.set_scale2f(TILE_SCALE, TILE_SCALE);
                sprite.set_position2f(TILE_SCALE * size.x * (x + 1) as f32 / 2.,
                                      TILE_SCALE * size.y * (y + 1) as f32 / 2.);
                println!("pos: {:?}", sprite.position());

                /*
                let cr = ((x + 5) as u8) * 25;
                let cg = ((y + 5) as u8) * 25;
                sprite.set_color(&Color::rgb(cr, cg, 255));
                */
                
                tiles.push(Tile { sprite });
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
