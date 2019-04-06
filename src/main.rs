// Draw some multi-colored geometry to the screen
use std::io::prelude::*;

extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Background::Img, Color, Font, FontStyle, Image, View},
    input::Key,
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};

use osu_format::{Beatmap, HitObject};

type MapObj = [bool; 7];

struct Camera {
    beatmap: Beatmap,
    speed: f64,
    duration: u32,
    map: Vec<MapObj>,
    position: f64,
    score: i64,
    play300: f64,
    asset: Asset<Image>,
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

        let asset = Asset::new(Image::load("hit300.png"));

        Ok(Camera {
            beatmap,
            speed: 0.2,
            duration,
            map,
            position: 0.0,
            score: 0,
            asset,
            play300: 0.0,
            buttons: [false, false, false, false, false, false, false],
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        // if window.keyboard()[Key::Left].is_down() {
        //     self.view = self.view.translate((-4, 0));
        // }
        // if window.keyboard()[Key::Right].is_down() {
        //     self.view = self.view.translate((4, 0));
        // }
        if window.current_fps() != 0.0 {
            self.position += 1000.0 / window.current_fps();
            if self.play300 > 0.0 {
                self.play300 -= 1000.0 / window.current_fps();
            }
        }
        if window.keyboard()[Key::S].is_down() {
            if !self.buttons[0] {
                let score = handle_keydown(&mut self.map, self.position, 0);
                self.score += score;
                if score == 300 {
                    self.play300 = 300.0;
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
                    self.play300 = 300.0;
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
                    self.play300 = 300.0;
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
                    self.play300 = 300.0;
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
                    self.play300 = 300.0;
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
                    self.play300 = 300.0;
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
                    self.play300 = 300.0;
                }
                self.buttons[6] = true;
            }
        } else {
            self.buttons[6] = false;
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let mut cur = 0;
        let mut cur_tp = &self.beatmap.timing_points[cur];
        for obj in 0..(1000.0 / self.speed) as usize {
            for (i, hit) in self.map[self.position as usize + obj as usize]
                .iter()
                .enumerate()
            {
                if *hit {
                    window.draw_ex(
                        &Rectangle::new(
                            (
                                (i as i32) * 73,
                                384 - (obj as f64 * (1000.0 / 384.0) * self.speed) as i32,
                            ),
                            (73, self.speed as f32 * cur_tp.milliseconds_per_beat / 8.0),
                        ),
                        Col(Color::BLUE),
                        Transform::scale((1, -1)),
                        1,
                    );
                }
            }
        }
        let play300 = self.play300;
        if self.play300 > 0.0 {
            self.asset.execute(|image| {
                window.draw_ex(
                    &image.area().with_center((256, 192)),
                    Img(&image),
                    Transform::scale((
                        1.0 - (((150.0 - play300).abs() as f32) / 150.0),
                        1.0 - (((150.0 - play300).abs() as f32) / 150.0),
                    )),
                    1,
                );
                Ok(())
            });
        }
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
