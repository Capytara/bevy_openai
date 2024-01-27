use bevy::prelude::{Commands, Component, Entity, Event, EventReader, EventWriter, Query};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use crate::ai_client::{ClientConfig, ClientConfigBuilder, send_to_ai_internal};

#[derive(Component)]
pub(crate) struct SendToAiTaskWrapper(pub Task<String>);

fn spawn_send_to_ai_task(commands: &mut Commands, prompt: String, client_config: ClientConfig) {
    let async_thread_pool = AsyncComputeTaskPool::get();
    let send_to_ai_task = async_thread_pool.spawn(async move { send_to_ai_internal(prompt, client_config) });
    commands.spawn(SendToAiTaskWrapper(send_to_ai_task));
}

#[derive(Event)]
pub struct SendToAiEvent(pub String);

#[derive(Event)]
pub struct SendToAiWithConfigEvent {
    pub prompt: String,
    pub config: ClientConfig,
}

pub(crate) fn process_send_to_ai_event(
    mut send_to_ai_events: EventReader<SendToAiEvent>,
    mut send_to_ai_with_config_events: EventReader<SendToAiWithConfigEvent>,
    mut commands: Commands,
) {
    for event in send_to_ai_events.read() {
        spawn_send_to_ai_task(
            &mut commands,
            event.0.clone(),
            ClientConfigBuilder::default()
                .build()
                .expect("Failed to build client config"),
        );
    }

    for event in send_to_ai_with_config_events.read() {
        spawn_send_to_ai_task(&mut commands, event.prompt.clone(), event.config.clone());
    }
}

#[derive(Event)]
pub struct AiResponseEvent(pub String);

pub(crate) fn process_ai_response(
    mut ai_response_event_writer: EventWriter<AiResponseEvent>,
    mut ai_response_tasks: Query<(Entity, &mut SendToAiTaskWrapper)>,
    mut commands: Commands,
) {
    for (entity, mut task) in &mut ai_response_tasks {
        if let Some(response_str) = future::block_on(future::poll_once(&mut task.0)) {
            ai_response_event_writer.send(AiResponseEvent(response_str));
            commands.entity(entity).despawn();
        }
    }
}