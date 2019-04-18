use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Img, Image},
    lifecycle::{Asset, Window},
    Result,
};

pub struct Number {
    digits: Vec<Asset<Image>>,
}

impl Number {
    pub fn new() -> Result<Number> {
        let mut digits = Vec::new();
        for i in 0..10 {
            digits.push(Asset::new(Image::load(format!(
                "static/skin/score-{}.png",
                i
            ))));
        }
        Ok(Number { digits })
    }

    pub fn draw(&mut self, window: &mut Window, pos: &Vector, size: &Vector, number: u32) {
        let digits: Vec<_> = number
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        for (i, digit) in digits.iter().enumerate() {
            self.digits[*digit as usize].execute(|image| {
                window.draw(
                    &Rectangle::new(
                        (pos.x + i as f32 * image.area().size.x, pos.y),
                        (image.area().size.x, image.area().size.y),
                    ),
                    Img(&image),
                );
                Ok(())
            });
        }
    }
}
