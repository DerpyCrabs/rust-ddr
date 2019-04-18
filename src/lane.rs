use crate::hit_score::HitResult;
use osu_format::HitObject;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{
        Background::{Col, Img},
        Color, Image,
    },
    input::Key,
    lifecycle::{Asset, Window},
    Result,
};

fn distance_to_hit_result(od: f32, distance: u32) -> HitResult {
    if (distance as f64) < (50.0 + 30.0 * (5.0 - od) / 5.0) as f64 {
        HitResult::Hit300
    } else if (distance as f64) < (100.0 + 40.0 * (5.0 - od) / 5.0) as f64 {
        HitResult::Hit100
    } else if (distance as f64) < (150.0 + 50.0 * (5.0 - od) / 5.0) as f64 {
        HitResult::Hit50
    } else {
        HitResult::Miss
    }
}

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
    od: f32,
}

impl Lane {
    pub fn new(lane_skin: LaneSkin, lane_map: &[HitObject], hotkey: Key, od: f32) -> Result<Lane> {
        let lane_skin_suffix = match lane_skin {
            LaneSkin::Lane1 => "1",
            LaneSkin::Lane2 => "2",
            LaneSkin::LaneS => "S",
        };
        let asset_key = Asset::new(Image::load(format!(
            "static/skin/mania-key{}.png",
            lane_skin_suffix
        )));
        let asset_key_down = Asset::new(Image::load(format!(
            "static/skin/mania-key{}D.png",
            lane_skin_suffix
        )));
        let asset_note = Asset::new(Image::load(format!(
            "static/skin/mania-note{}.png",
            lane_skin_suffix
        )));
        let asset_slider_body = Asset::new(Image::load(format!(
            "static/skin/mania-note{}L.png",
            lane_skin_suffix
        )));
        let asset_slider_end = Asset::new(Image::load(format!(
            "static/skin/mania-note{}H.png",
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
            od,
        })
    }

    pub fn update(&mut self, window: &mut Window, position: f32) -> HitResult {
        let is_pressed = self.is_pressed;
        let hotkey = self.hotkey;

        for i in self.lowest_index..self.map.len() {
            match &self.map[i] {
                // TODO calculate max hit distance using formula
                HitObject::Circle { base } => {
                    if (base.time as f64) < (position - 200.0) as f64 {
                        self.lowest_index += 1;
                        return HitResult::Miss;
                    } else {
                        break;
                    }
                }
                HitObject::LongNote { end_time, .. } => {
                    if (*end_time as f64) < (position - 200.0) as f64 {
                        self.lowest_index += 1;
                        return HitResult::Miss;
                    } else {
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }

        if window.keyboard()[hotkey].is_down() {
            if !is_pressed {
                self.is_pressed = true;
                let mut last_distance: Option<u32> = None;
                for i in self.lowest_index..self.map.len() {
                    let hit_object = &self.map[i];
                    // TODO handle long notes
                    if let HitObject::Circle { base } = hit_object {
                        let distance: u32 = ((base.time as i32) - (position as i32)).abs() as u32;
                        if let Some(last_dist) = last_distance {
                            if distance < last_dist {
                                last_distance = Some(distance);
                            } else {
                                let result = distance_to_hit_result(self.od, last_dist);
                                match result {
                                    HitResult::Miss => return result,
                                    _ => {
                                        self.lowest_index += 1;
                                        return result;
                                    }
                                }
                            }
                        } else {
                            last_distance = Some(distance);
                        }
                    }
                }
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
        key_height: f32,
        hit_line: f32,
    ) {
        let hit_objects = &mut self.map;
        let lowest_index = self.lowest_index;
        // TODO make note fall speed and note size somewhat predictable
        // TODO draw sliders
        if self.is_pressed {
            self.asset_key_down.execute(|key| {
                window.draw_ex(
                    &Rectangle::new((pos.x, pos.y + size.y - key_height), (size.x, key_height)),
                    Img(&key),
                    Transform::IDENTITY,
                    4,
                );
                Ok(())
            });
        } else {
            self.asset_key.execute(|key| {
                window.draw_ex(
                    &Rectangle::new((pos.x, pos.y + size.y - key_height), (size.x, key_height)),
                    Img(&key),
                    Transform::IDENTITY,
                    4,
                );
                Ok(())
            });
        }

        window.draw_ex(
            &Rectangle::new((pos.x, pos.y - hit_line + size.y), (size.x, 2.0)),
            Col(Color::RED),
            Transform::IDENTITY,
            5,
        );

        for i in lowest_index..hit_objects.len() {
            let hit_object = &hit_objects[i];
            match hit_object {
                HitObject::Circle { base } => {
                    if (base.time as f32 - position) * (speed / 100.0) > (size.y + 50.0) {
                        break;
                    }
                    self.asset_note.execute(|note| {
                        window.draw_ex(
                            &Rectangle::new(
                                (
                                    pos.x,
                                    pos.y - hit_line
                                        + (size.y
                                            - (base.time as f32 - position) * (speed / 100.0)),
                                ),
                                (size.x, speed / 4.0),
                            ),
                            Img(&note),
                            Transform::IDENTITY,
                            3,
                        );
                        Ok(())
                    });
                }
                HitObject::LongNote { base, end_time } => {
                    if (base.time as f32 - position) * (speed / 100.0) > (size.y + 50.0) {
                        break;
                    }
                    self.asset_slider_body.execute(|slider_body| {
                        window.draw_ex(
                            &Rectangle::new(
                                (
                                    pos.x,
                                    pos.y - hit_line
                                        + (size.y
                                            - (*end_time as f32 - position) * (speed / 100.0)),
                                ),
                                (size.x, (*end_time - base.time) as f32 * (speed / 100.0)),
                            ),
                            Img(&slider_body),
                            Transform::scale((1, -1)),
                            3,
                        );
                        Ok(())
                    });
                    self.asset_slider_end.execute(|slider_end| {
                        window.draw_ex(
                            &Rectangle::new(
                                (
                                    pos.x,
                                    pos.y - hit_line
                                        + (size.y
                                            - (base.time as f32 - position) * (speed / 100.0)),
                                ),
                                (size.x, speed / 4.0),
                            ),
                            Img(&slider_end),
                            Transform::IDENTITY,
                            3,
                        );
                        window.draw_ex(
                            &Rectangle::new(
                                (
                                    pos.x,
                                    pos.y - hit_line
                                        + (size.y
                                            - (*end_time as f32 - position) * (speed / 100.0)),
                                ),
                                (size.x, speed / 4.0),
                            ),
                            Img(&slider_end),
                            Transform::scale((1, -1)),
                            3,
                        );
                        Ok(())
                    });
                }
                _ => (),
            }
        }
    }
}
