use crate::hit_score::HitResult;
use osu_format::HitObject;
use quicksilver::{
    geom::{Shape, Transform, Vector},
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
}

impl Lane {
    pub fn new(lane_skin: LaneSkin, lane_map: &Vec<&HitObject>, hotkey: Key) -> Result<Lane> {
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
        })
    }

    pub fn update(&mut self, window: &mut Window) -> HitResult {
        if window.keyboard()[self.hotkey].is_down() {
            if !self.is_pressed {
                self.is_pressed = true;
                // handle keydown
                return HitResult::Miss;
            }
        } else {
            self.is_pressed = false;
        }
        return HitResult::NoHit;
    }

    pub fn draw(&mut self, window: &mut Window, center: Vector) {}
}
