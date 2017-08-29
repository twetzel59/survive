use resources::AnimatedTexture;

pub struct TextureAnimator {
    n_frames: u32,
    current: u32,
}

impl TextureAnimator {
    pub fn new(anim_tex: &AnimatedTexture) -> TextureAnimator {
        TextureAnimator {
            n_frames: anim_tex.n_frames,
            current: 0,
        }
    }
}
