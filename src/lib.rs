#![deny(clippy::all)]

use spinoff::{spinners, Color, Spinner};
use std::thread::sleep;
use std::time::Duration;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn compile() {
  let mut spinner: Spinner = Spinner::new(spinners::Dots, "Compiling...", Color::Blue);
  sleep(Duration::MAX);
  spinner.success("Done!");
}
