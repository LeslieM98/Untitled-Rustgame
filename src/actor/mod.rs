pub mod npc;
pub mod player;
pub mod target;

use crate::actor::target::{Target, Targetable};
use bevy::prelude::*;
use stats_and_abilities_system::prelude::{Health, StatBlock};
use std::time::SystemTime;

#[derive(Component)]
pub enum CombatStatus {
    InCombat,
    OutOfCombat,
}

impl Default for CombatStatus {
    fn default() -> Self {
        Self::OutOfCombat
    }
}

#[derive(Component)]
pub enum Relationship {
    Enemy,
    Neutral,
    Friend,
}

impl Default for Relationship {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Bundle, Default)]
pub struct Actor {
    pub name: Name,
    pub pbr: PbrBundle,
    pub stats: StatBlock,
    pub health: Health,
    pub combat_status: CombatStatus,
    pub target: Target,
    targetable: Targetable,
}

#[derive(Component)]
pub struct Name {
    pub value: String,
}

impl Default for Name {
    fn default() -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            value: now.to_string(),
        }
    }
}
