mod ai;

use crate::actor::npc::ai::AIPlugin;
use crate::actor::Actor;
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn).add_plugin(AIPlugin);
    }
}

#[derive(Component)]
pub struct NPCMarker;

#[derive(Bundle)]
pub struct Enemy {
    pub actor: Actor,
    pub marker: NPCMarker,
}

impl Enemy {
    pub fn from_pos(
        pos: Transform,
        meshes: &mut ResMut<Assets<Mesh>>,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Enemy {
        let tex_handle = asset_server.load("PNG/Red/texture_04.png");
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(tex_handle.clone()),
            unlit: false,
            ..default()
        });

        let pbr = PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: material_handle,
            transform: pos,
            ..default()
        };
        Enemy {
            actor: Actor { pbr, ..default() },
            ..default()
        }
    }
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            actor: Actor::default(),
            marker: NPCMarker,
        }
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let enemy1 = Enemy::from_pos(
        Transform::from_xyz(3.0, 1.0, 0.0),
        &mut meshes,
        &asset_server,
        &mut materials,
    );

    let enemy2 = Enemy::from_pos(
        Transform::from_xyz(-3.0, 1.0, 0.0),
        &mut meshes,
        &asset_server,
        &mut materials,
    );

    commands.spawn(enemy1);
    commands.spawn(enemy2);
}