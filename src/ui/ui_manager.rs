use sfml::graphics::*;
use super::Element;
use super::meter::Meter;
use resizehandler::ResizeHandler;
use resources::Resources;

pub struct UiManager<'s> {
    hydration: Meter<'s>,
}

impl<'s> UiManager<'s> {
    pub fn new(res: &'s Resources) -> UiManager<'s> {
        UiManager {
            hydration: Meter::new(res),
        }
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
