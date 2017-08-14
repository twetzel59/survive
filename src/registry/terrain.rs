pub mod colors {
    use sfml::graphics::Color;
    
    pub const GRASS: Color = Color { r:   25, g: 100, b:   25, a: 255 };
    pub const SAND: Color = Color { r: 255, g: 205, b: 85, a: 255 };
    pub const SAND_SPARKLE: Color = Color { r: 255, g: 245, b: 215, a: 255 };
    pub const WATER: Color = Color { r:  32, g:  64, b: 128, a: 255 };
    pub const WATER_DEEP: Color = Color { r:  16, g:  32, b: 72, a: 255 };
    pub const WATER_SPARKLE: Color = Color { r:  190, g:  215, b: 255, a: 255 };
}

#[derive(Clone, Copy, Debug)]
pub enum Terrain {
    Grass,
    Sand,
    Water,
}

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
