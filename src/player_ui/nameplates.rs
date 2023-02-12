use crate::actor::player::camera::PlayerCameraMarker;
use crate::player_ui::widgets::HealthBar;
use bevy::app::App;
use bevy::prelude::*;
use stats_and_abilities_system::prelude::{Health, StatBlock};

pub struct NamePlateUIPlugin;

#[derive(Component, Clone)]
pub struct NamePlateUIMarker;
#[derive(Component, Clone)]
pub struct NamePlateUIHealthBarMarker;

impl Plugin for NamePlateUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, clear_ui)
            .add_system(draw);
    }
}

fn instantiate(commands: &mut Commands, health: &Health, position: &Vec2, font: Handle<Font>) {
    HealthBar::new(NamePlateUIMarker, NamePlateUIHealthBarMarker)
        .with_width(100.0)
        .with_height(20.0)
        .with_pos_left(position.x)
        .with_pos_bottom(position.y)
        .with_background_color(Color::rgb(0.3, 0.3, 0.3))
        .with_health_color(Color::rgb(1.0, 0.3, 0.3))
        .with_font(Some(font))
        .draw(commands, health);
}

fn clear_ui(mut commands: Commands, ui_query: Query<Entity, With<NamePlateUIMarker>>) {
    for ui in &ui_query {
        commands.entity(ui).despawn_recursive();
    }
}

fn draw(
    mut commands: Commands,
    stat_query: Query<(&GlobalTransform, &Health)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCameraMarker>>,
    asset_server: Res<AssetServer>,
) {
    let (camera, camera_transform) = camera_query.get_single().expect("Player camera not found");
    for (actor_transform, actor_health) in &stat_query {
        if let Some(ui_position) =
            camera.world_to_viewport(camera_transform, actor_transform.translation())
        {
            let font = asset_server.load("fonts/FiraMono-Medium.ttf");
            instantiate(&mut commands, actor_health, &ui_position, font);
        }
    }
}
