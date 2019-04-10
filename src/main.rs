use std::io::prelude::*;
pub mod hit_score;
pub mod lane;

extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Background::Img, Color, Image},
    input::Key,
    lifecycle::{run, Asset, Settings, State, Window},
    sound::Sound,
    Result,
};

use crate::hit_score::{HitResult, HitScore};
use crate::lane::{Lane, LaneSkin};

use osu_format::{HitObject, TimingPoint};

#[derive(PartialEq, Clone, Copy)]
enum GameState {
    Playing,
    Paused,
}

struct Camera {
    timing_points: Vec<TimingPoint>,
    speed: f32,
    position: f32,
    score: u32,
    asset_bg: Asset<Image>,
    hit_score: HitScore,
    asset_music: Asset<Sound>,
    state: GameState,
    lanes: Vec<Lane>,
}

// TODO make this function lane count agnostic
fn x_to_lane(x: u32) -> usize {
    match x {
        36 => 0,
        109 => 1,
        182 => 2,
        256 => 3,
        329 => 4,
        402 => 5,
        475 => 6,
        _ => unimplemented!(),
    }
}

fn new_lanes(count: usize, lane_maps: Vec<Vec<HitObject>>, hotkeys: Vec<Key>) -> Result<Vec<Lane>> {
    let mut lanes: Vec<Result<Lane>> = Vec::new();
    for i in 0..count / 2 {
        let lane_skin = match i % 2 {
            0 => LaneSkin::Lane1,
            1 => LaneSkin::Lane2,
            _ => unreachable!(),
        };
        lanes.push(Lane::new(lane_skin, &lane_maps[i], hotkeys[i]));
    }
    for i in (count / 2)..count {
        let lane_skin = match i % 2 {
            0 => LaneSkin::Lane2,
            1 => LaneSkin::Lane1,
            _ => unreachable!(),
        };
        lanes.push(Lane::new(lane_skin, &lane_maps[i], hotkeys[i]));
    }
    if count % 2 == 1 {
        lanes[count / 2] = Lane::new(LaneSkin::LaneS, &lane_maps[count / 2], hotkeys[count / 2]);
    }
    lanes.into_iter().collect()
}

impl State for Camera {
    fn new() -> Result<Camera> {
        let f = std::fs::File::open("alice.osu").unwrap();
        let f = std::io::BufReader::new(f);
        let beatmap = osu_format::Parser::new(f.lines()).parse().unwrap();

        let lane_maps =
            beatmap
                .hit_objects
                .iter()
                .fold(vec![Vec::new(); 7], |mut acc, hit_object| {
                    match hit_object {
                        HitObject::Circle { base } => {
                            acc[x_to_lane(base.x)].push(hit_object.clone())
                        }
                        HitObject::LongNote { base, .. } => {
                            acc[x_to_lane(base.x)].push(hit_object.clone())
                        }
                        _ => (),
                    };
                    acc
                });

        let hotkeys = vec![Key::S, Key::D, Key::F, Key::Space, Key::J, Key::K, Key::L];

        let asset_bg = Asset::new(Image::load("bg.png"));
        let asset_music = Asset::new(Sound::load("music.mp3"));

        Ok(Camera {
            timing_points: beatmap.timing_points,
            speed: 0.4,
            position: 0.0,
            score: 0,
            hit_score: HitScore::new().unwrap(),
            asset_bg,
            asset_music,
            state: GameState::Paused,
            lanes: new_lanes(7, lane_maps, hotkeys).unwrap(),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Escape].is_down() {
            std::process::exit(0);
        }

        if self.position == 0.0 {
            if self.state == GameState::Paused {
                self.asset_music.execute(|sound| sound.play()).unwrap();
                self.state = GameState::Playing;
            }
        }

        if window.current_fps() != 0.0 {
            self.position += 1000.0 / window.current_fps() as f32;
        }

        let position = self.position;
        let results: Vec<HitResult> = self
            .lanes
            .iter_mut()
            .map(|lane| lane.update(window, position))
            .collect();
        results
            .iter()
            .for_each(|result| self.hit_score.play(*result));
        self.score += results
            .iter()
            .map(|result| match result {
                HitResult::Miss | HitResult::NoHit => 0,
                HitResult::Hit50 => 50,
                HitResult::Hit100 => 100,
                HitResult::Hit300 => 300,
            })
            .sum::<u32>();

        self.hit_score.update(window);

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.asset_bg
            .execute(|image| {
                window.draw_ex(
                    &image.area().with_center((960, 540)),
                    Img(&image),
                    Transform::scale((512.0 / image.area().size.x, 384.0 / image.area().size.y)),
                    -2,
                );
                Ok(())
            })
            .unwrap();
        window.draw_ex(
            &Rectangle::new((0, 0), (1920, 1080)),
            Col(Color::from_rgba(0, 0, 0, 0.8)),
            Transform::IDENTITY,
            -1,
        );

        let position = self.position;
        let speed = self.speed;
        let mpb = self.timing_points[0].milliseconds_per_beat;
        self.lanes.iter_mut().enumerate().for_each(|(i, lane)| {
            lane.draw(
                window,
                &Vector::new(704 + (i as i32) * 73, 348),
                &Vector::new(73, 384),
                position,
                speed * mpb,
            )
        });

        self.hit_score.draw(window, Vector::new(960, 540));
        Ok(())
    }
}

fn main() {
    run::<Camera>(
        "Camera",
        Vector::new(1920, 1080),
        Settings {
            vsync: false,
            fullscreen: true,
            ..Default::default()
        },
    );
}
