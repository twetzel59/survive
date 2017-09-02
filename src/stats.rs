#[derive(Debug)]
pub struct Stats {
    hydration: hydration::Hydration,
    temperature: temperature::Temperature,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            hydration: hydration::Hydration::new(),
            temperature: temperature::Temperature::new(),
        }
    }

    pub fn update(&mut self, delta: f32, daylight: f32) {
        self.hydration.update(delta, daylight);
        self.temperature.update(delta, daylight);
    }

    pub fn event(&mut self, event: &StatEvent) {
        self.hydration.handle(event);
        self.temperature.handle(event);
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
    InWater { delta: f32 },
    NearBonfire { delta: f32 },
}

trait StatComp {
    fn level(&self) -> f32;
    fn update(&mut self, delta: f32, daylight: f32);
    fn handle(&mut self, event: &StatEvent);
    fn fatal(&self) -> bool;
}

mod hydration {
    use super::*;

    #[derive(Debug)]
    pub struct Hydration {
        level: f32,
    }

    const MAX: f32 = 1.;
    const INC: f32 = 0.2;
    const DEC: f32 = 0.04;

    impl Hydration {
        pub fn new() -> Hydration {
            Hydration {
                level: MAX,
            }
        }
    }

    impl StatComp for Hydration {
        fn level(&self) -> f32 {
            self.level.max(0.).min(MAX)
        }

        fn update(&mut self, delta: f32, daylight: f32) {
            self.level -= delta * DEC * daylight * daylight;

            self.level = self.level.min(MAX);
            self.level = self.level.max(0.);
        }

        fn handle(&mut self, event: &StatEvent) {
            match *event {
                StatEvent::InWater { delta } => self.level += INC * delta,
                _ => {},
            };
        }

        fn fatal(&self) -> bool {
            self.level <= 0.
        }
    }
}

mod temperature {
    use super::*;

    #[derive(Debug)]
    pub struct Temperature {
        level: f32,
    }

    const MAX: f32 = 1.;
    const DAYLIGHT_INC_CUTOFF: f32 = 0.8;
    const DAYLIGHT_DEC_CUTOFF: f32 = 0.6;
    const INC_ENVIRONMENT: f32 = 0.01;
    const INC_BONFIRE: f32 = 0.12;
    const DEC: f32 = 0.08;

    impl Temperature {
        pub fn new() -> Temperature {
            Temperature {
                level: MAX,
            }
        }
    }

    impl StatComp for Temperature {
        fn level(&self) -> f32 {
            self.level.max(0.).min(MAX)
        }

        fn update(&mut self, delta: f32, daylight: f32) {
            if daylight < DAYLIGHT_DEC_CUTOFF {
                self.level -= DEC * delta;
            } else if daylight > DAYLIGHT_INC_CUTOFF {
                self.level += INC_ENVIRONMENT * delta;
            }
        }

        fn handle(&mut self, event: &StatEvent) {
            match *event {
                StatEvent::NearBonfire { delta } => self.level += INC_BONFIRE * delta,
                _ => {},
            };
        }

        fn fatal(&self) -> bool {
            self.level <= 0.
        }
    }
}
