use sfml::graphics::*;
use sfml::system::Vector2f;
use super::element::Element;
use super::meter::Meter;
use resize_handler::ResizeHandler;
use resources::Resources;
use stats::Stats;

pub struct UiManager<'s> {
    hydration: Meter<'s>,
}

impl<'s> UiManager<'s> {
    pub fn new(res: &'s Resources) -> UiManager<'s> {
        UiManager {
            hydration: Meter::new(res, &Vector2f::new(0.05, 0.02)),
        }
    }
    
    pub fn update(&mut self, current_stats: &Stats) {
        self.hydration.set_value(current_stats.hydration_level());
    }
    
    pub fn draw_all<T: RenderTarget>(&self, target: &mut T) {
        let old_view = target.view().to_owned();
        
        let default_view = target.default_view().to_owned();
        target.set_view(&default_view);
        
        self.hydration.draw(target);
        
        target.set_view(&old_view);
    }
}

impl<'s> ResizeHandler for UiManager<'s> {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.hydration.on_resize(width, height);
    }
}
