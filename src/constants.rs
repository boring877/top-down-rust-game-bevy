use bevy::prelude::*;

// ============================================================================
// GAME SETTINGS
// ============================================================================
pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const PLAYER_COLOR: Color = Color::srgb(0.3, 0.5, 0.9);
pub const PLAYER_HEALTH: u32 = 100;

// ============================================================================
// BOSS SETTINGS
// ============================================================================
pub const BOSS_SPEED: f32 = 80.0;
pub const BOSS_SIZE: Vec2 = Vec2::new(128.0, 128.0);
pub const BOSS_HEALTH: u32 = 500;
pub const BOSS_COLLIDER_RADIUS: f32 = 64.0;

// ============================================================================
// BULLET SETTINGS
// ============================================================================
pub const BULLET_SPEED: f32 = 400.0;
pub const BULLET_DAMAGE: u32 = 25;
pub const DEFAULT_FIRE_RATE: f32 = 0.2;
pub const SUPER_BULLET_SPEED: f32 = 300.0;
pub const SUPER_BULLET_DAMAGE: u32 = 100;
pub const SUPER_BULLET_INTERVAL: u32 = 4; // Every 4th shot is super

// ============================================================================
// FLOOR SETTINGS
// ============================================================================
pub const TILE_SIZE: f32 = 64.0;
pub const GRID_SIZE: i32 = 20;
pub const FLOOR_COLOR: Color = Color::srgb(0.0, 0.5, 0.0); // DEBUG: bright green
pub const GRID_COLOR: Color = Color::srgb(0.0, 0.3, 0.0); // DEBUG: dark green


// ============================================================================
// UI - SIZES
// ============================================================================
pub const BUTTON_WIDTH: f32 = 200.0;
pub const BUTTON_HEIGHT: f32 = 60.0;

// ============================================================================
// UI - FONT SIZES
// ============================================================================
pub const FONT_SIZE_TITLE: f32 = 60.0;
pub const FONT_SIZE_SPLASH: f32 = 80.0;
pub const FONT_SIZE_LABEL: f32 = 40.0;
pub const FONT_SIZE_BUTTON: f32 = 40.0;

// ============================================================================
// UI - COLORS
// ============================================================================
pub const COLOR_START_BUTTON: Color = Color::srgb(0.2, 0.6, 0.2);
pub const COLOR_SETTINGS_BUTTON: Color = Color::srgb(0.6, 0.4, 0.2);
pub const COLOR_BACK_BUTTON: Color = Color::srgb(0.6, 0.2, 0.2);
pub const COLOR_NAME_BUTTON: Color = Color::srgb(0.2, 0.2, 0.6);
pub const COLOR_TEXT: Color = Color::WHITE;

// ============================================================================
// UI - SPACING
// ============================================================================
pub const ROW_GAP_LARGE: f32 = 30.0;
pub const ROW_GAP_SMALL: f32 = 20.0;

// ============================================================================
// TIMERS
// ============================================================================
pub const SPLASH_DURATION_SECS: f32 = 2.0;
pub const DAMAGE_NUMBER_DURATION: f32 = 0.8;
pub const DAMAGE_NUMBER_SPEED: f32 = 80.0;
pub const DAMAGE_NUMBER_FONT_SIZE: f32 = 24.0;
pub const DAMAGE_NUMBER_OFFSET_Y: f32 = 20.0;

// ============================================================================
// TEXT
// ============================================================================
pub const GAME_TITLE: &str = "Player VS Boss";
