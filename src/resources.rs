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
                nyan: Texture::from_file("test.png").unwrap(),
            },
            fnt: Fonts {
                normal: Font::from_file("res/fnt/UbuntuMono-R.ttf").unwrap(),
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

pub struct Fonts {
    pub normal: Font,
}

pub struct Ui {
    pub hydration: Texture,
}
