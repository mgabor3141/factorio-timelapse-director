use ggez::mint::Point2;

// Game constants
pub const TILE_SIZE_PX: u32 = 32;
#[allow(dead_code)]
pub const TICK_PER_S: u32 = 60;
#[allow(dead_code)]
pub const MINIMUM_ZOOM: f32 = 0.3125; // Game limit

// Parameters
pub const RESOLUTION: Point2<u32> = Point2 { x: 3840, y: 2160 };
#[allow(dead_code)]
pub const CAMERA_TARGET_MINIMUM_ZOOM: f32 = MINIMUM_ZOOM * 6.0;
pub const CAMERA_TARGET_MAXIMUM_ZOOM: f32 = 1.0;
#[allow(dead_code)]
pub const CAMERA_FOLLOW_THRESHOLD: f64 = 20.0; // Distance in tiles
