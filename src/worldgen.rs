use noise::{NoiseModule, Perlin, Seedable};
use rand::{Rng, SeedableRng, XorShiftRng};
use sfml::graphics::*;
use registry::terrain::Terrain;
use tiles::TILES_ROW;

pub const WORLD_SIZE: u32 = 512;
const TILE_SIZE: i32 = WORLD_SIZE as i32 / TILES_ROW as i32;
const NOISE_INPUT_DIVISOR: f32 = 256.;

pub struct Worldgen {
    perlin: Perlin,
    rng: XorShiftRng,
    textures: Vec<Texture>,
    world: Vec<Terrain>,
}

impl Worldgen {
    pub fn new() -> Worldgen {
        let mut perlin = Perlin::new();
        perlin = perlin.set_seed(1);
        
        let mut wg = Worldgen {
            perlin,
            rng: SeedableRng::from_seed([1, 2, 3, 4]),
            textures: Vec::new(),
            world: Vec::new(),
        };
        
        wg.generate();
        
        wg
    }
    
    pub fn textures(&self) -> &[Texture] {
        &self.textures
    }
    
    pub fn world(&self) -> &[Terrain] {
        &self.world
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
                
                let value = self.perlin.get([fx / NOISE_INPUT_DIVISOR,
                                            fy / NOISE_INPUT_DIVISOR]);
                
                let (mut terrain, mut color) = if value > 0.45 {
                    (Terrain::Water, colors::WATER)
                } else {
                    (Terrain::Grass, colors::GRASS)
                };
                
                if value < 0.45 && value > 0.35 {
                    terrain = Terrain::Sand;
                    color = colors::SAND;
                }
                
                match terrain {
                    Terrain::Water => {
                        if x % 16 == 0 && y % 16 == 0 && self.perlin.get([fx / 7., fy / 7.]) > 0.6 {
                            color.r = (colors::WATER_SPARKLE.r as f32 * value) as u8;
                            color.g = (colors::WATER_SPARKLE.g as f32 * value) as u8;
                            color.b = (colors::WATER_SPARKLE.b as f32 * value) as u8;
                        } else if value > 0.57 {
                            color = colors::WATER_DEEP;
                        }
                    },
                    Terrain::Sand => {
                        if x % 8 == 0 && y % 8 == 0 && self.perlin.get([fx / 14., fy / 14.]) > 0.4 {
                            color = colors::SAND_SPARKLE;
                        }
                    },
                    _ => {},
                }
                
                image.set_pixel(x, y, &color);
                self.world.push(terrain);
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
