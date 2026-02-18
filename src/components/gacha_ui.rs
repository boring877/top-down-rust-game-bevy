use bevy::prelude::*;

// ============================================================================
// GACHA MENU ROOT
// ============================================================================
#[derive(Component)]
pub struct GachaMenuUI;

// ============================================================================
// NAVIGATION BAR
// ============================================================================
#[derive(Component)]
pub struct NavBar;

#[derive(Component)]
pub struct NavButton;

#[derive(Component)]
pub struct HomeButton;

#[derive(Component)]
pub struct ShopButton;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct CollectionButton;

#[derive(Component)]
pub struct GachaSettingsButton;

// ============================================================================
// CURRENCY DISPLAY
// ============================================================================
#[derive(Component)]
pub struct CurrencyBar;

#[derive(Component)]
pub struct CoinDisplay;

#[derive(Component)]
pub struct GemDisplay;

// ============================================================================
// SHOWCASE AREA
// ============================================================================
#[derive(Component)]
pub struct ShowcaseArea;

#[derive(Component)]
pub struct FeaturedBoss;

#[derive(Component)]
pub struct PlayerPreview;

// ============================================================================
// TOAST NOTIFICATION
// ============================================================================
#[derive(Component)]
pub struct ToastNotification {
    pub timer: Timer,
}

// ============================================================================
// GLOW EFFECT
// ============================================================================
#[derive(Component)]
pub struct GlowEffect {
    pub base_color: Color,
    pub glow_color: Color,
    pub time: f32,
}
