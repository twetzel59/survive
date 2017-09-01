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
                                delay: 0.22,
                                randomize: (true, true),
                            },
                //nyan: Texture::from_file("test.png").unwrap(),
            },
            fnt: Fonts {
                normal: Font::from_file("res/fnt/UbuntuMono-R.ttf").unwrap(),
            },
            ui: Ui {
                hydration: Texture::from_file("res/ui/hydration.png").unwrap(),
                temperature: Texture::from_file("res/ui/temperature.png").unwrap(),
                wood: Texture::from_file("res/ui/wood.png").unwrap(),
                day_night: Texture::from_file("res/ui/sundial.png").unwrap(),
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
    pub delay: f32,
    /// (x, y)
    pub randomize: (bool, bool)
}

pub struct Fonts {
    pub normal: Font,
}

pub struct Ui {
    pub hydration: Texture,
    pub temperature: Texture,
    pub wood: Texture,
    pub day_night: Texture,
}
