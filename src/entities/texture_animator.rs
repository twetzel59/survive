use resources::AnimatedTexture;
use rand::{Rng, thread_rng};
use sfml::graphics::IntRect;

pub struct TextureAnimator {
    n_frames: u32,
    step: i32,
    randomize: (bool, bool),
    tex_width: u32,
    current: u32,
    delay: f32,
    timer: f32,
    rect: IntRect,
}

impl TextureAnimator {
    pub fn new(anim_tex: &AnimatedTexture) -> TextureAnimator {
        let size = anim_tex.tex.size();

        let mut t = TextureAnimator {
            n_frames: anim_tex.n_frames,
            step: (size.y / anim_tex.n_frames) as i32,
            randomize: anim_tex.randomize,
            tex_width: size.x,
            current: 0,
            delay: anim_tex.delay,
            timer: 0.,
            rect: IntRect::new(0, 0, 1, 1),
        };

        t.gen_rect();

        t
    }

    pub fn update(&mut self, delta: f32) {
        self.timer += delta;
        if self.timer > self.delay {
            self.timer = 0.;
            self.gen_rect();

            self.current += 1;
            if self.current > self.n_frames - 1 {
                self.current = 0;
            }
        }
    }

    pub fn texture_rect(&self) -> IntRect {
        self.rect
    }

    fn gen_rect(&mut self) {
        let (left, width) = if self.randomize.0 && thread_rng().gen_weighted_bool(2) {
            (self.tex_width as i32, -(self.tex_width as i32))
        } else {
            (0, self.tex_width as i32)
        };

        let (top, height) = if self.randomize.1 && thread_rng().gen_weighted_bool(2) {
            ((self.current as i32 + 1) * self.step, -self.step)
        } else {
            (self.current as i32 * self.step, self.step)
        };

        self.rect = IntRect::new(left, top, width, height);
    }

    /*
    pub fn n_frames(&self) -> u32 {
        self.n_frames
    }

    pub fn current(&self) -> u32 {
        self.current
    }
    */
}
