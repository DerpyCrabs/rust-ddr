#![feature(clamp)]
use std::io::prelude::*;
use std::path::Path;
pub mod hit_score;
pub mod lane;
pub mod number;

extern crate quicksilver;

use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Background::Img, Color, Image},
    input::Key,
    lifecycle::{run, Asset, Settings, State, Window},
    sound::Sound,
    Result,
};

use crate::hit_score::{HitResult, HitScore};
use crate::lane::{Lane, LaneSkin};
use crate::number::Number;

use osu_format::{Event, HitObject, TimingPoint};

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
    number: Number,
    lanes: Vec<Lane>,
}

fn x_to_lane(x: u32, lane_count: u32) -> usize {
    (x as f32 / (512.0 / lane_count as f32))
        .floor()
        .clamp(0.0, (lane_count - 1) as f32) as usize
}

fn new_lanes(
    count: usize,
    lane_maps: Vec<Vec<HitObject>>,
    hotkeys: Vec<Key>,
    od: f32,
) -> Result<Vec<Lane>> {
    let mut lanes: Vec<Result<Lane>> = Vec::new();
    for i in 0..count / 2 {
        let lane_skin = match i % 2 {
            0 => LaneSkin::Lane1,
            1 => LaneSkin::Lane2,
            _ => unreachable!(),
        };
        lanes.push(Lane::new(lane_skin, &lane_maps[i], hotkeys[i], od));
    }
    for i in (count / 2)..count {
        let lane_skin = match i % 2 {
            0 => LaneSkin::Lane2,
            1 => LaneSkin::Lane1,
            _ => unreachable!(),
        };
        lanes.push(Lane::new(lane_skin, &lane_maps[i], hotkeys[i], od));
    }
    if count % 2 == 1 {
        lanes[count / 2] = Lane::new(
            LaneSkin::LaneS,
            &lane_maps[count / 2],
            hotkeys[count / 2],
            od,
        );
    }
    lanes.into_iter().collect()
}

impl State for Camera {
    fn new() -> Result<Camera> {
        std::env::set_current_dir(std::env::current_dir().unwrap().parent().unwrap()).unwrap();
        let map_path = std::env::args().collect::<Vec<String>>()[1].clone();

        let f = std::fs::File::open(&map_path).unwrap();
        let f = std::io::BufReader::new(f);
        let beatmap = osu_format::Parser::new(f.lines()).parse().unwrap();
        let note_count = beatmap.difficulty.circle_size as u32;
        let bg = if let Event::BackgroundMedia { filepath } = beatmap
            .events
            .iter()
            .find(|event| {
                if let Event::BackgroundMedia { .. } = event {
                    true
                } else {
                    false
                }
            })
            .unwrap()
        {
            Path::new(&map_path)
                .parent()
                .map(|par| par.join(filepath).to_string_lossy().into_owned())
                .unwrap_or(filepath.to_string())
        } else {
            unreachable!()
        };
        let music = Path::new(&map_path)
            .parent()
            .map(|par| {
                par.join(&beatmap.general.audio_filename)
                    .to_string_lossy()
                    .into_owned()
            })
            .unwrap_or(beatmap.general.audio_filename.to_string());
        let lane_maps = beatmap.hit_objects.iter().fold(
            vec![Vec::new(); note_count as usize],
            |mut acc, hit_object| {
                match hit_object {
                    HitObject::Circle { base } => {
                        acc[x_to_lane(base.x, note_count)].push(hit_object.clone())
                    }
                    HitObject::LongNote { base, .. } => {
                        acc[x_to_lane(base.x, note_count)].push(hit_object.clone())
                    }
                    _ => (),
                };
                acc
            },
        );

        let hotkeys = if note_count == 7 {
            vec![Key::S, Key::D, Key::F, Key::Space, Key::J, Key::K, Key::L]
        } else {
            vec![Key::D, Key::F, Key::J, Key::K]
        };

        let asset_bg = Asset::new(Image::load(bg.clone()));
        let asset_music = Asset::new(Sound::load(music.clone()));

        let od = beatmap.difficulty.overall_difficulty;
        Ok(Camera {
            timing_points: beatmap.timing_points,
            speed: 0.35,
            position: 0.0,
            score: 0,
            hit_score: HitScore::new().unwrap(),
            asset_bg,
            asset_music,
            number: Number::new().unwrap(),
            state: GameState::Paused,
            lanes: new_lanes(note_count as usize, lane_maps, hotkeys, od).unwrap(),
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
        let Vector { x: w, y: h } = window.screen_size();

        self.asset_bg
            .execute(|image| {
                window.draw_ex(
                    &image.area().with_center((w / 2.0, h / 2.0)),
                    Img(&image),
                    Transform::scale((w / image.area().size.x, h / image.area().size.y)),
                    -2,
                );
                Ok(())
            })
            .unwrap();
        window.draw_ex(
            &Rectangle::new((0, 0), (w, h)),
            Col(Color::from_rgba(0, 0, 0, 0.8)),
            Transform::IDENTITY,
            -1,
        );

        let position = self.position;
        let speed = self.speed;
        let mpb = self.timing_points[0].milliseconds_per_beat;
        let lane_count = self.lanes.len();
        self.lanes.iter_mut().enumerate().for_each(|(i, lane)| {
            lane.draw(
                window,
                &Vector::new(
                    (w as i32 / 2) - ((lane_count as f32 * 0.5) * 73.0) as i32
                        + (i as f32 * 73.0) as i32,
                    0,
                ),
                &Vector::new(72, h),
                position,
                speed * mpb,
                250.0,
                106.0,
            )
        });

        for i in 0..(self.lanes.len() + 1) {
            window.draw(
                &Line::new(
                    (
                        (w as i32 / 2) - ((lane_count as f32 * 0.5) * 73.0) as i32
                            + (i as f32 * 73.0) as i32,
                        0,
                    ),
                    (
                        (w as i32 / 2) - ((lane_count as f32 * 0.5) * 73.0) as i32
                            + (i as f32 * 73.0) as i32,
                        h - 106.0,
                    ),
                ),
                Col(Color::from_rgba(255, 255, 255, 0.4)),
            );
        }

        self.hit_score.draw(window, Vector::new(w / 2.0, h / 2.0));

        self.number.draw(
            window,
            &Vector::new(0, 0),
            &Vector::new(5, 5),
            window.current_fps() as u32,
        );
        self.number.draw(
            window,
            &Vector::new(0, 200),
            &Vector::new(5, 5),
            self.score as u32,
        );

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
            update_rate: 1.,
            max_updates: 1,
            draw_rate: 1.,
            ..Default::default()
        },
    );
}
