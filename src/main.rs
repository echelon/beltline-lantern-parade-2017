// Copyright (c) 2017 Brandon Thomas <bt@brand.io>
// 2017 Atlanta Beltline Lantern Parade Laser Demo

extern crate argparse;
extern crate beam;
extern crate ilda;
extern crate lase;

mod error;

use error::ParadeError;
use ilda::animation::Animation;

fn main() {
  let animation = read("./ild/font_impact.ild").expect("Couldn't read animation");
}

fn read(filename: &str) -> Result<Animation, ParadeError> {
  let animation = Animation::read_file(filename)?;
  Ok(animation)
}
