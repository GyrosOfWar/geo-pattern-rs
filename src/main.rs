#![feature(slicing_syntax)]

use svg::Svg;
use std::collections::HashMap;

pub mod svg;
pub mod pattern;

fn main() {
	let mut svg = Svg::new(800u, 800u);
	let mut args = HashMap::new();
	args.insert("fill".to_string(), "rgb(255, 0, 0)".to_string());
	svg.rectangle(30, 30, 60, 60, &args);
    println!("{}", svg);
}
