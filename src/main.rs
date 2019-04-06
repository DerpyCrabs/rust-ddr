// Draw some multi-colored geometry to the screen
use std::io::prelude::*;

extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Line, Rectangle, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Color, View},
    input::{Key},
    lifecycle::{Settings, State, Window, run},
};

use osu_format::{Beatmap, HitObjectBase, HitObject};

type MapObj = [bool; 7];

struct Camera {
    view: Rectangle,
    beatmap: Beatmap,
    speed: i32,
    duration: u32,
    map: Vec<MapObj>,
    current: i32,
}

fn x_to_note(x: u32) -> MapObj {
  match x {
    36 => [true, false, false, false, false, false, false],
    109 =>[false, true, false, false, false, false, false],
    182 =>[false, false, true, false, false, false, false],
    256 =>[false, false, false, true, false, false, false],
    329 =>[false, false, false, false, true, false, false],
    402 =>[false, false, false, false, false, true, false],
    475 =>[false, false, false, false, false, false, true],
    _ => [false, false, false, false, false, false, false]
  }
}
fn new_map(beatmap: &Beatmap) -> Vec<MapObj> {
    let duration = match beatmap.hit_objects.last().unwrap() {
      HitObject::Circle{base} => base.time,
      _ => 0,
    } + 100;
    
    let mut map = Vec::new();
    for tick in (0..duration) {
      map.push([false, false, false, false, false, false, false]);
    }
    
    for obj in beatmap.hit_objects.iter() {
      match obj {
        HitObject::Circle{base} => map[base.time as usize] = x_to_note(base.x),
        HitObject::LongNote{base, end_time} => {for i in (base.time..*end_time) {
          map[i as usize] = x_to_note(base.x);
        }
        },
        _ => ()
      }
    }
    
    return map;
}

impl State for Camera {
    fn new() -> Result<Camera> {
    let f = std::fs::File::open("map2.osu").unwrap();
    let f = std::io::BufReader::new(f);
    let beatmap = osu_format::Parser::new(f.lines()).parse().unwrap();
    let map = new_map(&beatmap);
    let duration = match beatmap.hit_objects.last().unwrap() {
      HitObject::Circle{base} => base.time,
      _ => 0,
    } + 100;
    
        Ok(Camera {
            view: Rectangle::new_sized((800, 600)),
    beatmap,
    speed: 10,
    duration, map,
    current: 0
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        // if window.keyboard()[Key::Left].is_down() {
        //     self.view = self.view.translate((-4, 0));
        // }
        // if window.keyboard()[Key::Right].is_down() {
        //     self.view = self.view.translate((4, 0));
        // }
        if window.keyboard()[Key::Down].is_down() {
          self.current += self.speed;
          self.view = self.view.translate((0, 4));
        }
        if window.keyboard()[Key::Up].is_down() {
          self.current -= self.speed;
          self.view = self.view.translate((0, -4));
        }
        window.set_view(View::new(self.view));
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
      let wind = 600 / self.speed;
        window.clear(Color::WHITE)?;
        for obj in (self.current..(self.current + wind)) {
          for (i, hit) in self.map[obj as usize].iter().enumerate() {
            if (*hit) {
            window.draw_ex(&Rectangle::new(((i as i32) * 73, 600/ ((obj as i32) / self.speed)), (73, 600/ wind)), Col(Color::BLUE), Transform::scale((1, -1)), 1);
            }
          }
        }
        Ok(())
    }
}


fn main() {
    run::<Camera>("Camera", Vector::new(800, 600), Settings::default());
}
