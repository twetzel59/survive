use std::f32::consts::PI;
use sfml::graphics::*;
use resize_handler::ResizeHandler;

const DAY_LENGTH: f32 = 20.;
const MAX_DARK: f32 = 128.;

pub struct DayNight {
    time: f32,
    fader: RectangleShape<'static>,
}

impl DayNight {
    pub fn new() -> DayNight {
        DayNight {
            time: 0.,
            fader: RectangleShape::new(),
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.time += delta * (1. / DAY_LENGTH);

        if self.time > 1. {
            self.time = self.time - 1.;
        }

        let light = self.daylight();
        self.fader.set_fill_color(&light);
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    fn daylight(&self) -> Color {
        let value = if self.time < 0.5 {
            self.time
        } else {
            1.0 - self.time
        }
            * 2.;

        //println!("{}        {}", self.time, (value * 2. * PI));
        //println!("value: {}", value);
        //println!("light: {}", (value * PI * 0.5).sin());
        //println!("time: {}", self.time);

        //Color::rgba(0, 0, 0, (((value * PI * 2.).sin() * 255.) / 2.) as u8)
        Color::rgba(0, 0, 0, ((value * PI * 0.5).sin() * MAX_DARK) as u8)
    }
}

impl Drawable for DayNight {
    fn draw<'se, 'tex, 'sh, 'shte>(
                                   &'se self,
                                   target: &mut RenderTarget,
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh
    {
        let old_view = target.view().to_owned();

        let size = target.size();
        let mut view = target.view().to_owned();
        view.set_center2f(size.x as f32 / 2., size.y as f32 / 2.);
        target.set_view(&view);

        target.draw_rectangle_shape(&self.fader, states);

        target.set_view(&old_view);
    }
}

impl ResizeHandler for DayNight {
    fn on_resize(&mut self, width: u32, height: u32) {
        let width = width as f32;
        let height = height as f32;

        self.fader.set_size2f(width, height);
    }
}
