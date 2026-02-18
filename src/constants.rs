use bevy::prelude::*;

// ============================================================================
// GAME SETTINGS
// ============================================================================
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const PLAYER_COLOR: Color = Color::srgb(0.3, 0.5, 0.9);
pub const PLAYER_HEALTH: u32 = 100;

// ============================================================================
// BOSS SETTINGS
// ============================================================================
pub const BOSS_SPEED: f32 = 300.0;
pub const BOSS_SIZE: Vec2 = Vec2::new(128.0, 128.0);
pub const BOSS_HEALTH: u32 = 500;
pub const BOSS_COLLIDER_RADIUS: f32 = 64.0;
pub const BOSS_ATTACK_COOLDOWN: f32 = 2.0;
pub const BOSS_BLADE_DAMAGE: u32 = 20;
pub const BOSS_BLADE_DURATION: f32 = 1.5;
pub const BOSS_BLADE_RANGE: f32 = 60.0;
pub const BOSS_BLADE_SPEED: f32 = 400.0;
pub const BOSS_BLADE_SIZE: f32 = 80.0;

// ============================================================================
// TEST SETTINGS
// ============================================================================
pub const TEST_MODE: bool = true;
pub const BOSS_SPAWN_INTERVAL: f32 = 3.0; // Spawn boss every 3 seconds

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
// ARENA SETTINGS
// ============================================================================
pub const ARENA_HALF_WIDTH: f32 = 1920.0;  // 3840 total width (covers 4K width)
pub const ARENA_HALF_HEIGHT: f32 = 1200.0; // 2400 total height (covers most screens)
pub const FLOOR_COLOR: Color = Color::srgb(0.25, 0.28, 0.32); // Modern dark slate

// Arena obstacle positions (x, y, shape: 0=rock, 1=crystal, 2=pillar)
// Spread across the full arena (-1920 to 1920 x, -1200 to 1200 y)
pub const ARENA_OBSTACLES: [(f32, f32, i32); 24] = [
    // Rocks around perimeter
    (-1400.0, 900.0, 0),
    (1400.0, 900.0, 0),
    (-1400.0, -900.0, 0),
    (1400.0, -900.0, 0),
    (-700.0, 1000.0, 0),
    (700.0, 1000.0, 0),
    (-700.0, -1000.0, 0),
    (700.0, -1000.0, 0),
    // Crystals in strategic positions
    (-1200.0, 0.0, 1),
    (1200.0, 0.0, 1),
    (0.0, 800.0, 1),
    (0.0, -800.0, 1),
    // Pillars creating cover
    (-600.0, 500.0, 2),
    (600.0, 500.0, 2),
    (-600.0, -500.0, 2),
    (600.0, -500.0, 2),
    (-300.0, 0.0, 2),
    (300.0, 0.0, 2),
    (0.0, 300.0, 2),
    (0.0, -300.0, 2),
    // Additional obstacles for larger arena
    (-1000.0, 500.0, 2),
    (1000.0, 500.0, 2),
    (-1000.0, -500.0, 2),
    (1000.0, -500.0, 2),
];


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
// OBSTACLE SETTINGS
// ============================================================================
pub const OBSTACLE_SIZE: f32 = 64.0;
pub const OBSTACLE_ROCK_COLOR: Color = Color::srgb(0.55, 0.45, 0.38);
pub const OBSTACLE_CRYSTAL_COLOR: Color = Color::srgb(0.4, 0.6, 0.9);
pub const OBSTACLE_PILLAR_COLOR: Color = Color::srgb(0.6, 0.55, 0.5);

// ============================================================================
// TEXT
// ============================================================================
pub const GAME_TITLE: &str = "Player VS Boss";

// ============================================================================
// GACHA MENU - SIZES
// ============================================================================
pub const NAVBAR_HEIGHT: f32 = 80.0;
pub const NAV_BUTTON_SIZE: f32 = 60.0;
pub const PLAY_BUTTON_SIZE: f32 = 90.0;
pub const CURRENCY_BAR_HEIGHT: f32 = 50.0;
pub const SHOWCASE_HEIGHT: f32 = 300.0;
pub const TOAST_DURATION: f32 = 2.0;

// ============================================================================
// GACHA MENU - COLORS
// ============================================================================
pub const COLOR_PLAY_BUTTON: Color = Color::srgb(0.9, 0.7, 0.2);  // Gold
pub const COLOR_PLAY_BUTTON_HOVER: Color = Color::srgb(1.0, 0.85, 0.4);
pub const COLOR_NAVBAR_BG: Color = Color::srgb(0.08, 0.08, 0.12);
pub const COLOR_NAV_BUTTON: Color = Color::srgb(0.15, 0.15, 0.2);
pub const COLOR_NAV_BUTTON_HOVER: Color = Color::srgb(0.25, 0.25, 0.35);
pub const COLOR_CURRENCY_BG: Color = Color::srgb(0.1, 0.1, 0.15);
pub const COLOR_SHOWCASE_BG: Color = Color::srgb(0.12, 0.12, 0.18);
pub const COLOR_MENU_BG: Color = Color::srgb(0.06, 0.06, 0.1);
pub const COLOR_GLOW: Color = Color::srgb(1.0, 0.85, 0.3);
pub const COLOR_COIN: Color = Color::srgb(1.0, 0.8, 0.2);
pub const COLOR_GEM: Color = Color::srgb(0.5, 0.8, 1.0);
pub const COLOR_TOAST_BG: Color = Color::srgba(0.2, 0.2, 0.3, 0.9);

// ============================================================================
// GACHA MENU - FONT SIZES
// ============================================================================
pub const FONT_SIZE_NAV: f32 = 18.0;
pub const FONT_SIZE_CURRENCY: f32 = 20.0;
pub const FONT_SIZE_SHOWCASE_TITLE: f32 = 32.0;
pub const FONT_SIZE_TOAST: f32 = 24.0;
