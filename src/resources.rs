use sfml::graphics::Texture;

pub struct Resources {
    pub img: Images,
    pub ui: Ui,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            img: Images {
                player: Texture::from_file("res/img/player.png").unwrap(),
                nyan: Texture::from_file("test.png").unwrap(),
            },
            ui: Ui {
                hydration: Texture::from_file("res/ui/hydration.png").unwrap(),
            },
        }
    }
}

pub struct Images {
    pub player: Texture,
    pub nyan: Texture,
}

pub struct Ui {
    pub hydration: Texture,
}
