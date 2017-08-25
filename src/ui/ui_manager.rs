use sfml::graphics::*;
use sfml::system::Vector2f;
use super::death_screen::DeathScreen;
use super::element::*;
use super::meter::Meter;
use resize_handler::ResizeHandler;
use resources::Resources;
use stats::Stats;

pub struct UiManager<'s> {
    hydration: Meter<'s>,
    death: DeathScreen<'s>,
    display_death: bool,
}

impl<'s> UiManager<'s> {
    pub fn new(res: &'s Resources) -> UiManager<'s> {
        UiManager {
            hydration: Meter::new(res, &Vector2f::new(0.05, 0.02)),
            death: DeathScreen::new(res),
            display_death: false,
        }
    }

    pub fn update(&mut self, delta: f32, current_stats: &Stats) {
        self.hydration.set_value(current_stats.hydration_level());
        if self.display_death {
            self.death.update(delta);
        }
    }

    pub fn draw_all<T: RenderTarget>(&self, target: &mut T) {
        let old_view = target.view().to_owned();

        let default_view = target.default_view().to_owned();
        target.set_view(&default_view);

        self.hydration.draw(target);
        self.death.draw(target);

        target.set_view(&old_view);
    }

    pub fn set_display_death(&mut self, display_death: bool) {
        self.display_death = display_death;
    }
}

impl<'s> ResizeHandler for UiManager<'s> {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.hydration.on_resize(width, height);
        self.death.on_resize(width, height);
    }
}
