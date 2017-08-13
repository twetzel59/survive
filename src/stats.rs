#[derive(Debug)]
pub struct Stats {
    hydration: Hydration,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            Hydration::new(),
        }
    }
}

pub enum StatEvent {
    InWater,
}

trait StatComp {
    fn update(&mut self, delta: f32);
    fn handle(&mut self, event: &StatEvent, delta: f32);
}

struct Hydration {
    level: f32,
}

impl Hydration {
    fn new() -> Hydration {
        Hydration {
            level: 500.,
        }
    }
}

impl StatComp for Hydration {
    fn update(&mut self, delta: f32) {
        self.level -= delta;
    }
    
    fn handle(&mut self, event: &StatEvent, delta: f32) {
        match event {
            StatEvent::InWater => self.level += 10 * delta;
            _ => {},
        };
        
        if self.level > 500. {
            self.level = 500.;
        }
    }
}
