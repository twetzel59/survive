use rand::{Rng, SeedableRng, XorShiftRng};
use entities::deciduous_tree::DeciduousTree;
use entities::entity_manager::EntityManager;
use resources::Resources;
use tiles::TILE_SCALE;
use worldgen::WORLD_SIZE;

const BOUND: i32 = WORLD_SIZE as i32 / 105;
const INTERVAL: f32 = 64.;

//use worldgen::{PLANT_SECTORS, PLANT_SKIP, PLANT_SKIP_HALF,
//               WORLD_SIZE, WORLD_SIZE_HALF};

//const INTERVAL: i32 = (PLANT_SKIP as f32 * TILE_SCALE * 64.) as i32;
//const OFFSET: i32 = ((WORLD_SIZE_HALF as f32 * TILE_SCALE) / 2.) as i32;

pub fn generate_plants<'a>(res: &'a Resources, em: &mut EntityManager<'a>) {
    let mut xor: XorShiftRng = SeedableRng::from_seed([1, 2, 3, 4]);
    
    for x in (-BOUND + 1)..BOUND {
        for y in (-BOUND + 1)..BOUND {
            em.add(DeciduousTree::with_position2f(res,
                                                  x as f32 * TILE_SCALE * INTERVAL,
                                                  y as f32 * TILE_SCALE * INTERVAL));
        }
    }
    
    /*
    for x in 0..(WORLD_SIZE / PLANT_SECTORS) as i32 {
        for y in 0..(WORLD_SIZE / PLANT_SECTORS) as i32 {
            //match xor.gen_range(0u32, 5) {
            //    0 => 
            //}
            
            em.add(DeciduousTree::with_position2f(res,
                                                  (x * INTERVAL - OFFSET) as f32,
                                                  (y * INTERVAL - OFFSET) as f32));
        }
    }
    */
}
