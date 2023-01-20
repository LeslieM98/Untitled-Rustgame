use crate::actor::health::Health;
use bevy::app::App;
use bevy::prelude::*;
use std::ops::Deref;

pub struct StatusEventPlugin;
impl Plugin for StatusEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init)
            .add_system_to_stage(CoreStage::PostUpdate, resolve_events);
    }
}

#[derive(Component, Default)]
pub struct ActionReceivedEventQueue {
    pub events: Vec<ActionReceivedEvent>,
}

pub struct ActionReceivedEvent {
    pub apply: Box<dyn Fn() -> () + Sync + Send>,
}

fn resolve_events(
    mut affected_query: Query<(Entity, &ActionReceivedEventQueue)>,
    mut commands: Commands,
) {
    for (entity, event_queue) in affected_query.iter_mut() {
        for event in &event_queue.events {
            event.apply.deref()();
        }
        commands
            .entity(entity)
            .insert(ActionReceivedEventQueue::default());
    }
}

fn init(mut commands: Commands, health_queries: Query<Entity, With<Health>>) {
    for entity in &health_queries {
        commands
            .entity(entity)
            .insert(ActionReceivedEventQueue::default());
    }
}
