use ggez::mint::Point2;

// Game constants
pub const TILE_SIZE_PX: u32 = 32;
#[allow(dead_code)]
pub const TICK_PER_S: u32 = 60;
#[allow(dead_code)]
pub const MINIMUM_ZOOM: f32 = 0.3125;

// Parameters
pub const RESOLUTION: Point2<u32> = Point2 { x: 3840, y: 2160 };
