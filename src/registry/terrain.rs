use sfml::graphics::Color;

pub const GRASS: Color = Color { r:   0, g: 128, b:   0, a: 255 };
pub const WATER: Color = Color { r:  32, g:  64, b: 128, a: 255 };

/*
pub struct Spec {
    pub color: Color,
}

pub struct Registry {
    pub grass: Spec,
    pub water: Spec,
}

pub const TERRAIN: Registry = Registry {
    grass: 
};
*/

/*
pub struct Spec {
    pub color: Color,
}

pub enum Terrain {
    Grass(Spec),
    Water(Spec),
}

pub struct Registry {
    pub grass: Terrain,
    pub water: Terrain,
}

pub const TERRAIN: Registry = Registry {
    grass: Terrain::Grass(Spec { color: Color { r: 0, g: 128, b: 0, a: 255 } }),
    water: Terrain::Water(Spec { color: Color { r: 32, g: 64, b: 128, a: 255 } }),
};
*/
