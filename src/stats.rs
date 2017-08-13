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

const HYDRATION_MAX: f32 = 100.;
const HYDRATION_INC: f32 = 10.;

impl Hydration {
    fn new() -> Hydration {
        Hydration {
            level: HYDRATION_MAX,
        }
    }
}

impl StatComp for Hydration {
    fn update(&mut self, delta: f32) {
        self.level -= delta;
        
        if self.level > HYDRATION_MAX {
            self.level = HYDRATION_MAX;
        }
    }
    
    fn handle(&mut self, delta: f32, event: &StatEvent) {
        match *event {
            StatEvent::InWater => self.level += HYDRATION_INC * delta,
        };
    }
}
