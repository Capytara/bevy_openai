pub mod ai_client;
pub mod ai_ecs;

use ai_ecs::{
    process_ai_response, process_send_to_ai_event, AiResponseEvent, SendToAiEvent,
    SendToAiWithConfigEvent,
};
use bevy::{
    app::{Plugin, Update},
    ecs::{component::Component, event::Event},
};

pub struct OpenAiPlugin;

impl Plugin for OpenAiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SendToAiEvent>()
            .add_event::<AiResponseEvent>()
            .add_event::<SendToAiWithConfigEvent>()
            .add_systems(Update, process_send_to_ai_event)
            .add_systems(Update, process_ai_response);
    }
}
