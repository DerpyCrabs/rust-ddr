use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Image},
    lifecycle::{Asset, Window},
    Result,
};

#[derive(Copy, Clone)]
pub enum HitResult {
    NoHit,
    Miss,
    Hit50,
    Hit100,
    Hit300,
}

pub struct HitScore {
    playing: Option<(HitResult, f32)>,
    asset_miss: Asset<Image>,
    asset_hit50: Asset<Image>,
    asset_hit100: Asset<Image>,
    asset_hit300: Asset<Image>,
    animation_duration: f32,
}

impl HitScore {
    pub fn new() -> Result<HitScore> {
        let asset_hit300 = Asset::new(Image::load("static/skin/hit300.png"));
        let asset_hit100 = Asset::new(Image::load("static/skin/hit100.png"));
        let asset_hit50 = Asset::new(Image::load("static/skin/hit50.png"));
        let asset_miss = Asset::new(Image::load("static/skin/hit0.png"));

        Ok(HitScore {
            playing: None,
            asset_miss,
            asset_hit50,
            asset_hit100,
            asset_hit300,
            animation_duration: 300.0,
        })
    }

    pub fn play(&mut self, hit_result: HitResult) {
        if let HitResult::NoHit = hit_result {
            return;
        }
        self.playing = Some((hit_result, self.animation_duration));
    }

    pub fn update(&mut self, window: &mut Window) {
        if let Some((hit_result, animation_progress)) = self.playing {
            if window.current_fps() != 0.0 {
                if animation_progress > 0.0 {
                    self.playing = Some((
                        hit_result,
                        animation_progress - 1000.0 / window.current_fps() as f32,
                    ));
                } else {
                    self.playing = None;
                }
            }
        }
    }

    pub fn draw(&mut self, window: &mut Window, center: Vector) {
        if let Some((hit_result, animation_progress)) = self.playing {
            let asset = match hit_result {
                HitResult::Miss => &mut self.asset_miss,
                HitResult::Hit50 => &mut self.asset_hit50,
                HitResult::Hit100 => &mut self.asset_hit100,
                HitResult::Hit300 => &mut self.asset_hit300,
                _ => unreachable!(),
            };
            let animation_duration = self.animation_duration;
            asset
                .execute(|image| {
                    window.draw_ex(
                        &image.area().with_center(center),
                        Img(&image),
                        Transform::scale((
                            1.0 - (((animation_duration / 2.0 - animation_progress).abs() as f32)
                                / (animation_duration / 2.0)),
                            1.0 - (((animation_duration / 2.0 - animation_progress).abs() as f32)
                                / (animation_duration / 2.0)),
                        )),
                        5,
                    );
                    Ok(())
                })
                .expect("Failed to draw hit score");
        }
    }
}
