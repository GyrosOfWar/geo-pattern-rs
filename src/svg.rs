use std::fmt;
use std::collections::HashMap;

pub struct Svg {
	data: String,
	width: uint,
	height: uint
}

impl Svg {
	pub fn new(width: uint, height: uint) -> Svg {
		Svg {
			data: String::new(),
			width: width,
			height: height
		}
	}

	fn header(&self) -> String {
		format!("<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}'>", self.width, self.height)
	}

	fn footer(&self) -> String {
		"</svg>".to_string()
	}

	fn format_args(args: &HashMap<String, String>) -> String {
		let mut result = String::new();
		for (k, v) in args.iter() {
			result.push_str(format!("{}='{}'' ", k, v).as_slice());
		}

		result
	}

	pub fn rectangle(&mut self, x: uint, y: uint, w: uint, h: uint, args: &HashMap<String, String>) {
		let rect_string = format!("<rect x='{}' y='{}' width='{}' height='{}' {} />",
			x, y, w, h, Svg::format_args(args));

		self.data.push_str(rect_string.as_slice());
	}

	pub fn circle(&mut self, cx: uint, cy: uint, r: uint, args: &HashMap<String, String>) {
		let circle_string = format!("<circle cx='{}' cy='{}' r='{}' {} />",
			cx, cy, r, Svg::format_args(args));

		self.data.push_str(circle_string.as_slice());
	}

	pub fn path(&mut self, d: String, args: &HashMap<String, String>) {
		let path_string = format!("<path d='{}' {} />",
			d, Svg::format_args(args));

		self.data.push_str(path_string.as_slice());
	}

	pub fn polyline(&mut self, d: String, args: &HashMap<String, String>) {
		let polyline_string = format!("<polyline points='{}' {} />",
			d, Svg::format_args(args));

		self.data.push_str(polyline_string.as_slice());
	}

	pub fn group(&mut self, e1: &str, e2: &str, args: &HashMap<String, String>) {
		let opening_tag = format!("<g {}>", Svg::format_args(args));

		self.data.push_str(opening_tag.as_slice());
		self.data.push_str(e1);
		self.data.push_str(e2);
		self.data.push_str("</g>");
	}
}

impl fmt::Show for Svg {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "{}{}{}", self.header(), self.data, self.footer())
	}
}

#[test]
fn test_empty_svg() {
	let svg = Svg::new(200u, 200u);
	let svg_str = svg.to_string();
	let result = "<svg xmlns='http://www.w3.org/2000/svg' width='200' height='200'></svg>".to_string();

	assert_eq!(svg_str, result);
}