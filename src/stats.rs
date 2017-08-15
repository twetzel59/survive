#[derive(Debug)]
pub struct Stats {
    hydration: Hydration,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            hydration: Hydration::new(),
        }
    }
    
    pub fn update(&mut self, delta: f32) {
        self.hydration.update(delta);
    }
    
    pub fn event(&mut self, delta: f32, event: &StatEvent) {
        self.hydration.handle(delta, event);
    }
    
    pub fn hydration_level(&self) -> f32 {
        self.hydration.level()
    }
}

pub enum StatEvent {
    InWater,
}

trait StatComp {
    fn update(&mut self, delta: f32);
    fn handle(&mut self, delta: f32, event: &StatEvent);
}

#[derive(Debug)]
struct Hydration {
    level: f32,
}

const HYDRATION_MAX: f32 = 1.;
const HYDRATION_INC: f32 = 1.0;
const HYDRATION_DEC: f32 = 0.1;

impl Hydration {
    fn new() -> Hydration {
        Hydration {
            level: HYDRATION_MAX,
        }
    }
    
    fn level(&self) -> f32 {
        self.level
    }
}

impl StatComp for Hydration {
    fn update(&mut self, delta: f32) {
        self.level -= delta * HYDRATION_DEC;
        
        self.level = self.level.min(HYDRATION_MAX);
        self.level = self.level.max(0.);
    }
    
    fn handle(&mut self, delta: f32, event: &StatEvent) {
        match *event {
            StatEvent::InWater => self.level += HYDRATION_INC * delta,
        };
    }
}
