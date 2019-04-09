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

use osu_format::{Beatmap, HitObject};

type MapObj = [bool; 7];

#[derive(PartialEq, Clone, Copy)]
enum GameState {
    Playing,
    Paused,
}

struct Camera {
    beatmap: Beatmap,
    speed: f64,
    duration: u32,
    map: Vec<MapObj>,
    position: f64,
    score: i64,
    asset_note1: Asset<Image>,
    asset_note2: Asset<Image>,
    asset_noteS: Asset<Image>,
    asset_bg: Asset<Image>,
    hit_score: HitScore,
    asset_music: Asset<Sound>,
    state: GameState,
    buttons: [bool; 7],
}

fn x_to_note(x: u32) -> MapObj {
    match x {
        36 => [true, false, false, false, false, false, false],
        109 => [false, true, false, false, false, false, false],
        182 => [false, false, true, false, false, false, false],
        256 => [false, false, false, true, false, false, false],
        329 => [false, false, false, false, true, false, false],
        402 => [false, false, false, false, false, true, false],
        475 => [false, false, false, false, false, false, true],
        _ => [false, false, false, false, false, false, false],
    }
}
fn new_map(beatmap: &Beatmap) -> Vec<MapObj> {
    let duration = match beatmap.hit_objects.last().unwrap() {
        HitObject::Circle { base } => base.time,
        _ => 0,
    } + 100;

    let mut map = Vec::new();
    for _ in 0..duration {
        map.push([false, false, false, false, false, false, false]);
    }

    for obj in beatmap.hit_objects.iter() {
        match obj {
            HitObject::Circle { base } => map[base.time as usize] = x_to_note(base.x),
            HitObject::LongNote { base, end_time } => {
                for i in base.time..*end_time {
                    map[i as usize] = x_to_note(base.x);
                }
            }
            _ => (),
        }
    }

    return map;
}

fn handle_keydown(map: &mut Vec<MapObj>, position: f64, index: usize) -> i64 {
    let mut found = false;
    for obj in (position as i32 - 400)..(position as i32 + 400) {
        if map[obj as usize][index] == true {
            found = true;
            map[obj as usize][index] = false;
            break;
        }
    }
    if found {
        return 300;
    }
    return 0;
}

impl State for Camera {
    fn new() -> Result<Camera> {
        let f = std::fs::File::open("alice.osu").unwrap();
        let f = std::io::BufReader::new(f);
        let beatmap = osu_format::Parser::new(f.lines()).parse().unwrap();
        let map = new_map(&beatmap);
        let duration = match beatmap.hit_objects.last().unwrap() {
            HitObject::Circle { base } => base.time,
            _ => 0,
        } + 100;

        let asset_note1 = Asset::new(Image::load("skin/mania-note1.png"));
        let asset_note2 = Asset::new(Image::load("skin/mania-note2.png"));
        let asset_noteS = Asset::new(Image::load("skin/mania-noteS.png"));
        let asset_bg = Asset::new(Image::load("bg.png"));
        let asset_music = Asset::new(Sound::load("music.mp3"));

        Ok(Camera {
            beatmap,
            speed: 0.2,
            duration,
            map,
            position: 0.0,
            score: 0,
            hit_score: HitScore::new().unwrap(),
            asset_note1,
            asset_note2,
            asset_noteS,
            asset_bg,
            asset_music,
            state: GameState::Paused,
            buttons: [false, false, false, false, false, false, false],
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.current_fps() != 0.0 {
            self.position += 1000.0 / window.current_fps();
        }
        self.hit_score.update(window);
        if self.position == 0.0 {
            if self.state == GameState::Paused {
                self.asset_music.execute(|sound| sound.play());
                self.state = GameState::Playing;
            }
        }
        if window.keyboard()[Key::S].is_down() {
            if !self.buttons[0] {
                let score = handle_keydown(&mut self.map, self.position, 0);
                self.score += score;
                if score == 300 {
                    self.hit_score.play(HitResult::Hit300);
                }
                self.buttons[0] = true;
            }
        } else {
            self.buttons[0] = false;
        }
        if window.keyboard()[Key::D].is_down() {
            if !self.buttons[1] {
                let score = handle_keydown(&mut self.map, self.position, 1);
                self.score += score;
                if score == 300 {
                    self.hit_score.play(HitResult::Hit300);
                }
                self.buttons[1] = true;
            }
        } else {
            self.buttons[1] = false;
        }
        if window.keyboard()[Key::F].is_down() {
            if !self.buttons[2] {
                let score = handle_keydown(&mut self.map, self.position, 2);
                self.score += score;
                if score == 300 {
                    self.hit_score.play(HitResult::Hit300);
                }
                self.buttons[2] = true;
            }
        } else {
            self.buttons[2] = false;
        }
        if window.keyboard()[Key::Space].is_down() {
            if !self.buttons[3] {
                let score = handle_keydown(&mut self.map, self.position, 3);
                self.score += score;
                if score == 300 {
                    self.hit_score.play(HitResult::Hit300);
                }
                self.buttons[3] = true;
            }
        } else {
            self.buttons[3] = false;
        }
        if window.keyboard()[Key::J].is_down() {
            if !self.buttons[4] {
                let score = handle_keydown(&mut self.map, self.position, 4);
                self.score += score;
                if score == 300 {
                    self.hit_score.play(HitResult::Hit300);
                }
                self.buttons[4] = true;
            }
        } else {
            self.buttons[4] = false;
        }
        if window.keyboard()[Key::K].is_down() {
            if !self.buttons[5] {
                let score = handle_keydown(&mut self.map, self.position, 5);
                self.score += score;
                if score == 300 {
                    self.hit_score.play(HitResult::Hit300);
                }
                self.buttons[5] = true;
            }
        } else {
            self.buttons[5] = false;
        }
        if window.keyboard()[Key::L].is_down() {
            if !self.buttons[6] {
                let score = handle_keydown(&mut self.map, self.position, 6);
                self.score += score;
                if score == 300 {
                    self.hit_score.play(HitResult::Hit300);
                }
                self.buttons[6] = true;
            }
        } else {
            self.buttons[6] = false;
        }
        if window.keyboard()[Key::Escape].is_down() {
            std::process::exit(0);
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.asset_bg.execute(|image| {
            window.draw_ex(
                &image.area().with_center((256, 192)),
                Img(&image),
                Transform::scale((512.0 / image.area().size.x, 384.0 / image.area().size.y)),
                -2,
            );
            Ok(())
        });
        window.draw_ex(
            &Rectangle::new((0, 0), (512, 384)),
            Col(Color::from_rgba(0, 0, 0, 0.8)),
            Transform::IDENTITY,
            -1,
        );
        let mut cur = 0;
        let mut cur_tp = &self.beatmap.timing_points[cur];
        let speed = self.speed;
        let map = &self.map;
        let position = self.position;

        self.asset_note1.execute(|image| {
            for obj in 0..(1000.0 / speed) as usize {
                let map_obj = map[position as usize + obj as usize];
                if map_obj[0] {
                    window.draw_ex(
                        &Rectangle::new(
                            (
                                (0 as i32) * 73,
                                384 - (obj as f64 * (1000.0 / 384.0) * speed) as i32,
                            ),
                            (73, speed as f32 * cur_tp.milliseconds_per_beat / 8.0),
                        ),
                        Img(&image),
                        Transform::scale((1, -1)),
                        1,
                    );
                }
                if map_obj[6] {
                    window.draw_ex(
                        &Rectangle::new(
                            (
                                (6 as i32) * 73,
                                384 - (obj as f64 * (1000.0 / 384.0) * speed) as i32,
                            ),
                            (73, speed as f32 * cur_tp.milliseconds_per_beat / 8.0),
                        ),
                        Img(&image),
                        Transform::scale((1, -1)),
                        1,
                    );
                }
                if map_obj[2] {
                    window.draw_ex(
                        &Rectangle::new(
                            (
                                (2 as i32) * 73,
                                384 - (obj as f64 * (1000.0 / 384.0) * speed) as i32,
                            ),
                            (73, speed as f32 * cur_tp.milliseconds_per_beat / 8.0),
                        ),
                        Img(&image),
                        Transform::scale((1, -1)),
                        1,
                    );
                }
                if map_obj[4] {
                    window.draw_ex(
                        &Rectangle::new(
                            (
                                (4 as i32) * 73,
                                384 - (obj as f64 * (1000.0 / 384.0) * speed) as i32,
                            ),
                            (73, speed as f32 * cur_tp.milliseconds_per_beat / 8.0),
                        ),
                        Img(&image),
                        Transform::scale((1, -1)),
                        1,
                    );
                }
            }
            Ok(())
        });
        self.asset_note2.execute(|image| {
            for obj in 0..(1000.0 / speed) as usize {
                let map_obj = map[position as usize + obj as usize];
                if map_obj[1] {
                    window.draw_ex(
                        &Rectangle::new(
                            (
                                (1 as i32) * 73,
                                384 - (obj as f64 * (1000.0 / 384.0) * speed) as i32,
                            ),
                            (73, speed as f32 * cur_tp.milliseconds_per_beat / 8.0),
                        ),
                        Img(&image),
                        Transform::scale((1, -1)),
                        1,
                    );
                }
                if map_obj[5] {
                    window.draw_ex(
                        &Rectangle::new(
                            (
                                (5 as i32) * 73,
                                384 - (obj as f64 * (1000.0 / 384.0) * speed) as i32,
                            ),
                            (73, speed as f32 * cur_tp.milliseconds_per_beat / 8.0),
                        ),
                        Img(&image),
                        Transform::scale((1, -1)),
                        1,
                    );
                }
            }
            Ok(())
        });
        self.asset_noteS.execute(|image| {
            for obj in 0..(1000.0 / speed) as usize {
                let map_obj = map[position as usize + obj as usize];
                if map_obj[3] {
                    window.draw_ex(
                        &Rectangle::new(
                            (
                                (3 as i32) * 73,
                                384 - (obj as f64 * (1000.0 / 384.0) * speed) as i32,
                            ),
                            (73, speed as f32 * cur_tp.milliseconds_per_beat / 8.0),
                        ),
                        Img(&image),
                        Transform::scale((1, -1)),
                        1,
                    );
                }
            }
            Ok(())
        });

        self.hit_score.draw(window, Vector::new(256, 192));
        Ok(())
    }
}

fn main() {
    run::<Camera>(
        "Camera",
        Vector::new(512, 384),
        Settings {
            vsync: false,
            ..Default::default()
        },
    );
}
