#[derive(Debug)]
pub struct Stats {
    hydration: Hydration,
    temperature: Temperature,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            hydration: Hydration::new(),
            temperature: Temperature::new(),
        }
    }

    pub fn update(&mut self, delta: f32, day_time: f32) {
        self.hydration.update(delta, day_time);
        self.temperature.update(delta, day_time);
    }

    pub fn event(&mut self, delta: f32, event: &StatEvent) {
        self.hydration.handle(delta, event);
        self.temperature.handle(delta, event);
    }

    pub fn hydration_level(&self) -> f32 {
        self.hydration.level()
    }

    pub fn temperature_level(&self) -> f32 {
        self.temperature.level()
    }

    pub fn dead(&self) -> bool {
        self.hydration.fatal() || self.temperature.fatal()
    }
}

pub enum StatEvent {
    InWater,
}

trait StatComp {
    fn level(&self) -> f32;
    fn update(&mut self, delta: f32, day_time: f32);
    fn handle(&mut self, delta: f32, event: &StatEvent);
    fn fatal(&self) -> bool;
}

#[derive(Debug)]
struct Hydration {
    level: f32,
}

const HYDRATION_MAX: f32 = 1.;
const HYDRATION_INC: f32 = 0.2;
const HYDRATION_DEC: f32 = 0.04;

impl Hydration {
    fn new() -> Hydration {
        Hydration {
            level: HYDRATION_MAX,
        }
    }
}

impl StatComp for Hydration {
    fn level(&self) -> f32 {
        self.level.max(0.)
    }

    fn update(&mut self, delta: f32, _day_time: f32) {
        self.level -= delta * HYDRATION_DEC;

        self.level = self.level.min(HYDRATION_MAX);
        self.level = self.level.max(0.);
    }

    fn handle(&mut self, delta: f32, event: &StatEvent) {
        match *event {
            StatEvent::InWater => self.level += HYDRATION_INC * delta,
        };
    }

    fn fatal(&self) -> bool {
        self.level <= 0.
    }
}

#[derive(Debug)]
struct Temperature {
    level: f32,
}

const TEMPERATURE_MAX: f32 = 1.;
//const TEMPERATURE_INC
impl Temperature {
    fn new() -> Temperature {
        Temperature {
            level: TEMPERATURE_MAX,
        }
    }
}

impl StatComp for Temperature {
    fn level(&self) -> f32 {
        self.level.max(0.)
    }

    fn update(&mut self, _delta: f32, _day_time: f32) {
        self.level -= 0.01;
    }

    fn handle(&mut self, _delta: f32, _event: &StatEvent) {
    }

    fn fatal(&self) -> bool {
        self.level <= 0.
    }
}
