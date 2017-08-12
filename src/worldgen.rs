use noise::{NoiseModule, Perlin, Seedable};
use sfml::graphics::*;
use tiles::TILES_ROW;

const NOISE_INPUT_DIVISOR: f32 = 512.;
const WORLD_SIZE: u32 = 1024;
//const TILES: i32 = TILES_ROW * TILES_ROW;
const TILE_SIZE: i32 = WORLD_SIZE as i32 / TILES_ROW as i32;

pub struct WorldGen {
    perlin: Perlin,
    textures: Vec<Texture>,
}

impl WorldGen {
    pub fn new() -> WorldGen {
        let mut perlin = Perlin::new();
        perlin = perlin.set_seed(1);
        
        let mut wg = WorldGen {
            perlin,
            textures: Vec::new(),
        };
        
        wg.generate();
        
        wg
    }
    
    pub fn textures(&self) -> &[Texture] {
        &self.textures
    }
    
    fn generate(&mut self) {
        use registry::terrain::*;
        
        let mut image = Image::from_color(WORLD_SIZE,
                                          WORLD_SIZE,
                                          &Color::green())
                               .unwrap();
        
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                let fx = x as f32;
                let fy = y as f32;
                
                let color = if self.perlin.get(
                                [fx / NOISE_INPUT_DIVISOR,
                                fy / NOISE_INPUT_DIVISOR]) > 0.45 {
                    WATER
                } else {
                    GRASS
                };
                
                image.set_pixel(x, y, &color);
            }
        }
        
        self.split_into_textures(&image);
        
        //Texture::from_image(&image).unwrap()
    }
    
    fn split_into_textures(&mut self, image: &Image) {
        for x in 0..TILES_ROW {
            for y in 0..TILES_ROW {
                let tex = Texture::from_image_with_rect(
                                   image, &IntRect::new(x * TILE_SIZE,
                                                        y * TILE_SIZE,
                                                        TILE_SIZE,
                                                        TILE_SIZE))
                                   .unwrap();

                self.textures.push(tex);
            }
        }
    }
}
