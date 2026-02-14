use bevy::prelude::*;

#[derive(Component)]
pub struct SplashUI;

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);
