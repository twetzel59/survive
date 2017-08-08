use sfml::graphics::*;
use sfml::window::*;

const TITLE: &'static str = "#survive";
const DEFAULT_SIZE: (u32, u32) = (800, 600);

pub struct GameWindow {
    pub rwin: RenderWindow,
}

impl GameWindow {
    pub fn new() -> GameWindow {
        let mut rwin = RenderWindow::new(VideoMode::new(
                                            DEFAULT_SIZE.0, DEFAULT_SIZE.1, 32),
                                    TITLE,
                                    style::DEFAULT,
                                    &Default::default()).unwrap();
        rwin.set_vertical_sync_enabled(true);
        
        GameWindow {
            rwin,
        }
    }
}
