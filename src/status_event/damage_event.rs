use crate::actor::status::Stats;
use crate::status_event::TargetAssociation;
use bevy::prelude::{Commands, Component, Entity, Query, SystemSet, With};
use std::borrow::BorrowMut;

pub fn get_system_set() -> SystemSet {
    SystemSet::new().with_system(resolve_damage_events)
}

#[derive(Component, Default)]
pub struct DamageEventQueue {
    pub events: Vec<DamageEvent>,
}

pub struct DamageEvent {
    pub target_association: TargetAssociation,
    ///First stat struct is the source, second is the target
    pub apply: Box<dyn Fn(Stats, &mut Stats) + Sync + Send>,
}

pub fn resolve_damage_events(
    affected_query: Query<(Entity, &DamageEventQueue)>,
    mut stats_query: Query<&mut Stats>,
    mut commands: Commands,
) {
    for (entity, damage_queues) in affected_query.iter() {
        for event in damage_queues.events.iter() {
            let source_stats = stats_query
                .get(event.target_association.source)
                .expect(
                    format!("Cannot find source: {:?}", event.target_association.source).as_str(),
                )
                .clone();

            let mut target_stats = stats_query.get_mut(event.target_association.target).expect(
                format!("Cannot find target: {:?}", event.target_association.target).as_str(),
            );

            (event.apply)(source_stats, target_stats.borrow_mut());
        }
        commands.entity(entity).insert(DamageEventQueue::default());
    }
}

pub fn init(mut commands: Commands, health_queries: Query<Entity, With<Stats>>) {
    for entity in &health_queries {
        commands.entity(entity).insert(DamageEventQueue::default());
    }
}
