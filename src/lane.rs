use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Image},
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
}

impl Lane {
    pub fn new(lane_skin: LaneSkin) -> Result<Lane> {
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
        })
    }

    pub fn update(&mut self, window: &mut Window) {}

    pub fn draw(&mut self, window: &mut Window, center: Vector) {}
}
