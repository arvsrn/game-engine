pub type RGB = (u8, u8, u8);
pub type RGBA = (u8, u8, u8, u8);

pub fn mix(color1: RGB, color2: RGB, mix: f32) -> RGB {
    let mut blended: RGB = (0, 0, 0);

    blended.0 = (mix * color1.0 as f32 + (1.0 - mix) * color2.0 as f32) as u8;
    blended.1 = (mix * color1.1 as f32 + (1.0 - mix) * color2.1 as f32) as u8;
    blended.2 = (mix * color1.2 as f32 + (1.0 - mix) * color2.2 as f32) as u8;

    return blended;
}
