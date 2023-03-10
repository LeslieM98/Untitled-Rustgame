use crate::actor::player::PlayerMarker;
use crate::actor::target::Target;
use crate::settings::controls::ActionBarAction;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub fn player_action(
    actions: Query<&ActionState<ActionBarAction>>,
    player_queue: Query<(Entity, &Target), With<PlayerMarker>>,
) {
    for action in actions.iter() {
        for (_player_entity, player_target) in &player_queue {
            if action.just_pressed(ActionBarAction::Button1) {
                if let Some(_target_entity) = player_target.targeted_entity {}
            } else if action.just_pressed(ActionBarAction::Button2) {
                if let Some(_target_entity) = player_target.targeted_entity {}
            }
        }
    }
}
