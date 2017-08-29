use sfml::graphics::{Font, Texture};

pub struct Resources {
    pub img: Images,
    pub fnt: Fonts,
    pub ui: Ui,
}

impl Resources {
    pub fn new() -> Resources {
        let mut res = Resources {
            img: Images {
                player: Texture::from_file("res/img/player.png").unwrap(),
                deciduous: Texture::from_file("res/img/deciduous.png").unwrap(),
                bonfire: Texture::from_file("res/img/bonfire.png").unwrap(),
                fire_atlas: AnimatedTexture {
                                tex: Texture::from_file("res/img/fire_atlas.png").unwrap(),
                                n_frames: 4,
                            },
                //nyan: Texture::from_file("test.png").unwrap(),
            },
            fnt: Fonts {
                normal: Font::from_file("res/fnt/UbuntuMono-R.ttf").unwrap(),
            },
            ui: Ui {
                hydration: Texture::from_file("res/ui/hydration.png").unwrap(),
                wood: Texture::from_file("res/ui/wood.png").unwrap(),
            },
        };

        res.ui.hydration.set_smooth(true);

        res
    }
}

pub struct Images {
    pub player: Texture,
    pub deciduous: Texture,
    pub bonfire: Texture,
    pub fire_atlas: AnimatedTexture,
    //pub nyan: Texture,
}

pub struct AnimatedTexture {
    pub tex: Texture,
    pub n_frames: u32,
}

pub struct Fonts {
    pub normal: Font,
}

pub struct Ui {
    pub hydration: Texture,
    pub wood: Texture,
}
