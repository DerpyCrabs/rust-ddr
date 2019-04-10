use crate::hit_score::HitResult;
use osu_format::HitObject;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Img, Image},
    input::Key,
    lifecycle::{Asset, Window},
    Result,
};

pub enum LaneSkin {
    Lane1,
    Lane2,
    LaneS,
}

pub struct Lane {
    asset_key: Asset<Image>,
    asset_key_down: Asset<Image>,
    asset_note: Asset<Image>,
    asset_slider_body: Asset<Image>,
    asset_slider_end: Asset<Image>,
    is_pressed: bool,
    hotkey: Key,
    map: Vec<HitObject>,
    lowest_index: usize,
}

impl Lane {
    pub fn new(lane_skin: LaneSkin, lane_map: &[HitObject], hotkey: Key) -> Result<Lane> {
        let lane_skin_suffix = match lane_skin {
            LaneSkin::Lane1 => "1",
            LaneSkin::Lane2 => "2",
            LaneSkin::LaneS => "S",
        };
        let asset_key = Asset::new(Image::load(format!(
            "skin/mania-key{}.png",
            lane_skin_suffix
        )));
        let asset_key_down = Asset::new(Image::load(format!(
            "skin/mania-key{}D.png",
            lane_skin_suffix
        )));
        let asset_note = Asset::new(Image::load(format!(
            "skin/mania-note{}.png",
            lane_skin_suffix
        )));
        let asset_slider_body = Asset::new(Image::load(format!(
            "skin/mania-note{}L.png",
            lane_skin_suffix
        )));
        let asset_slider_end = Asset::new(Image::load(format!(
            "skin/mania-note{}H.png",
            lane_skin_suffix
        )));

        Ok(Lane {
            asset_key,
            asset_key_down,
            asset_note,
            asset_slider_body,
            asset_slider_end,
            is_pressed: false,
            hotkey,
            map: lane_map.to_vec(),
            lowest_index: 0,
        })
    }

    pub fn update(&mut self, window: &mut Window, position: f32) -> HitResult {
        if window.keyboard()[self.hotkey].is_down() {
            if !self.is_pressed {
                self.is_pressed = true;
                // TODO handle keydown
                return HitResult::Miss;
            }
        } else {
            self.is_pressed = false;
        }
        return HitResult::NoHit;
    }

    pub fn draw(
        &mut self,
        window: &mut Window,
        pos: &Vector,
        size: &Vector,
        position: f32,
        speed: f32,
    ) {
        let hit_objects = &mut self.map;
        // TODO make note fall speed and note size somewhat predictable
        // TODO clip map drawing
        // TODO draw keys
        // TODO draw sliders
        self.asset_note.execute(|note| {
            hit_objects.iter().for_each(|hit_object| {
                if let HitObject::Circle { base } = hit_object {
                    window.draw_ex(
                        &Rectangle::new(
                            (
                                pos.x,
                                (size.y - (base.time as f32 - position)) * (speed / 100.0),
                            ),
                            (size.x, speed / 4.0),
                        ),
                        Img(&note),
                        Transform::IDENTITY,
                        3,
                    );
                }
            });
            Ok(())
        });
    }
}
