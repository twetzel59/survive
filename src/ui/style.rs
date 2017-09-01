use sfml::graphics::Color;
use super::meter::MeterStyle;

pub const HYDRATION_METER: MeterStyle = MeterStyle {
    fill: Color { r: 18, g: 86, b: 153, a: 255 },
    outline: Color { r: 30, g: 144, b: 255, a: 255 },
};

pub const TEMPERATURE_METER: MeterStyle = MeterStyle {
    fill: Color { r: 153, g: 30, b: 18, a: 255 },
    outline: Color { r: 255, g: 40, b: 30, a: 255 },
};
