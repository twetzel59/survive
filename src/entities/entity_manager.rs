use sfml::graphics::{FloatRect, RenderTarget};
use sfml::system::{Vector2f, Vector2i};
use super::entity::{Entity, EntityKind};
use inventory::Inventory;

const WOOD_PER_BONFIRE: u8 = 10;
const MAX_REACH: f32 = 100.;
const MAX_BONFIRE_DIST: f32 = 150.;

pub struct EntityManager<'s> {
    entities: Vec<Box<Entity<'s> + 's>>,
}

impl<'s> EntityManager<'s> {
    pub fn new() -> EntityManager<'s> {
        //let mut entities: Vec<Box<Entity<'s>>> = Vec::new();
        //entities.push(Box::new(DeciduousTree::new(res)));

        EntityManager {
            entities: Vec::new(),
        }
    }

    pub fn draw_all<T: RenderTarget>(&self, target: &mut T, day_lights: bool) {
        for i in &self.entities {
            if i.daylight_affects() == day_lights {
                i.draw(target);
            }
        }
    }

    pub fn update(&mut self, delta: f32) {
        for i in &mut self.entities {
            i.update(delta);
        }
    }

    /*
    pub fn add<T: Entity<'s> + 'a>(&mut self, entity: T) {
        self.entities.push(Box::new(entity));
    }
    */

    /*
    pub fn add(&mut self, entity: Box<Entity<'s> + 's>) {
        self.entities.push(entity);
    }
    */

    pub fn add<T: Entity<'s> + 's>(&mut self, entity: T) {
        self.entities.push(Box::new(entity));
    }

    pub fn click(&mut self, player_pos: &Vector2f, inv: &mut Inventory,
                target: &RenderTarget, mx: i32, my: i32) {
        let coords = target.map_pixel_to_coords_current_view(&Vector2i::new(mx, my));

        println!("EntityManager click: {:?}", coords);

        for i in &mut self.entities {
            if !i.care_about_click() {
                println!("skip");
                continue;
            }

            //let bounds = i.sprite().global_bounds();
            let bounds = i.global_bounds();
            if bounds.contains(coords) {

                if Self::close(&bounds, player_pos, MAX_REACH) {
                    //println!("Dropped: {:?}", i.on_click());
                    inv.add(i.on_click());
                    /*
                    match i.on_click() {
                        Some(s) => inv.add(s),
                        None => false,
                    };
                    */
                }
            }
        }
    }

    pub fn near_bonfire(&self, player_pos: &Vector2f) -> bool {
        for i in &self.entities {
            match i.kind() {
                EntityKind::Bonfire => {},
                _ => continue,
            };

            if Self::close(&i.global_bounds(), player_pos, MAX_BONFIRE_DIST) {
                return true;
            }
        }

        false
    }

    pub fn spawn_bonfire(&mut self, res: &'s ::resources::Resources,
                         world: &[::registry::terrain::Terrain], player_pos: &Vector2f,
                         inv: &mut Inventory, target: &RenderTarget, mx: i32, my: i32) {
        use ::registry::terrain::Terrain;
        use ::registry::item::{Item, Stack};
        use super::bonfire::Bonfire;

        let coords = target.map_pixel_to_coords_current_view(&Vector2i::new(mx, my));

        let d = coords - *player_pos;

        //println!("{:?}", d);

        //if 

        if (d.x * d.x + d.y * d.y).sqrt() <= MAX_REACH {
            if inv.remove(Stack { item: Item::Wood, count: WOOD_PER_BONFIRE }) {
                self.add(Bonfire::with_position(res, &coords));
            }
        }
    }

    fn close(bounds: &FloatRect, pos: &Vector2f, max_dist: f32) -> bool {
        let (cx, cy) = (bounds.left + bounds.width / 2.,
                        bounds.top + bounds.height / 2.);

        let dx = cx - pos.x;
        let dy = cy - pos.y;

        (dx * dx + dy * dy).sqrt() <= max_dist
    }
}
