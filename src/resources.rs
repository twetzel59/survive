use sfml::graphics::Texture;

pub struct Resources {
    pub img: Images,
    pub ui: Ui,
}

impl Resources {
    pub fn new() -> Resources {
        let mut res = Resources {
            img: Images {
                player: Texture::from_file("res/img/player.png").unwrap(),
                deciduous: Texture::from_file("res/img/deciduous.png").unwrap(),
                nyan: Texture::from_file("test.png").unwrap(),
            },
            ui: Ui {
                hydration: Texture::from_file("res/ui/hydration.png").unwrap(),
            },
        };
        
        res.ui.hydration.set_smooth(true);
        
        res
    }
}

pub struct Images {
    pub player: Texture,
    pub deciduous: Texture,
    pub nyan: Texture,
}

pub struct Ui {
    pub hydration: Texture,
}
