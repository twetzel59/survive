use sfml::graphics::*;

const WORLD_SIZE: u32 = 1024;

pub struct WorldGen {
    image: Image,
}

impl WorldGen {
    pub fn new() -> WorldGen {
        WorldGen {
            image: Image::from_color(WORLD_SIZE,
                                     WORLD_SIZE,
                                     &Color::green())
                          .unwrap(),
        }
    }
    
    pub fn generate(mut self) -> Texture {
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                let color = if x % 2 == 0 {
                    Color::white()
                } else {
                    Color::black()
                };
                
                self.image.set_pixel(x, y, &color);
            }
        }
        
        Texture::from_image(&self.image).unwrap()
    }
}
