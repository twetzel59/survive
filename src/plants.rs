use rand::{Rng, SeedableRng, XorShiftRng};
use entities::deciduous_tree::DeciduousTree;
use entities::entity_manager::EntityManager;
use resources::Resources;
use registry::terrain::Terrain;
use tiles::TILE_SCALE;
use worldgen::{WORLD_SIZE, WORLD_SIZE_HALF};

const DIVISOR: i32 = 105;
const BOUND: i32 = WORLD_SIZE as i32 / DIVISOR;
const INTERVAL: f32 = 64.;

pub fn generate_plants<'a>(res: &'a Resources, world: &[Terrain], em: &mut EntityManager<'a>) {
    let mut xor: XorShiftRng = SeedableRng::from_seed([1, 2, 3, 4]);

    for x in (-BOUND + 1)..BOUND {
        for y in (-BOUND + 1)..BOUND {
            let wx = x as f32 * INTERVAL;
            let wy = y as f32 * INTERVAL;
            
            let ax = (wx + WORLD_SIZE_HALF as f32) as u32;
            let ay = (wy + WORLD_SIZE_HALF as f32) as u32;
            
            let index = (ax * WORLD_SIZE + ay) as usize;
            
            //println!("({}, {}) wx: {}, wy: {}, ax: {}, ay: {}, [{}]: {:?}",
            //         x, y, wx, wy, ax, ay,
            //         index, world[index]);
            
            match world[index] {
                Terrain::Grass => {
                    em.add(DeciduousTree::with_position2f(res,
                                                          wx * TILE_SCALE,
                                                          wy * TILE_SCALE));
                },
                _ => {},
            }
        }
    }
}
