use sfml::graphics::Texture;

pub struct Resources {
    pub img: Images,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            img: Images {
                player: Texture::from_file("res/img/player.png").unwrap(),
            },
        }
    }
}

pub struct Images {
    pub player: Texture,
}
