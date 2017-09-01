use sfml::graphics::Color;
use super::meter::MeterStyle;

pub const HYDRATION_METER: MeterStyle = MeterStyle {
    fill: Color { r: 18, g: 86, b: 153, a: 255 },
    outline: Color { r: 30, g: 144, b: 255, a: 255 },
};
