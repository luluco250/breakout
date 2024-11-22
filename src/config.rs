pub struct Config {
	pub width: u16,
	pub height: u16,
	pub vsync: bool,
	pub fps_cap: Option<u16>,
}

impl Config {
	pub fn read_config() -> Self {
		Self {
			width: 640,
			height: 480,
			vsync: true,
			fps_cap: None,
		}
	}
}
