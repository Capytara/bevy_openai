use bevy::prelude::*;
use bevy_openai::ai_ecs::{AiResponseEvent, SendToAiEvent};
use bevy_openai::OpenAiPlugin;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, OpenAiPlugin))
        .add_systems(Startup, send_to_ai)
        .add_systems(Update, process_ai_response)
        .run();
}

fn send_to_ai(
    mut event_writer: EventWriter<SendToAiEvent>
) {
    event_writer.send(SendToAiEvent("Hello".to_string()));
}

fn process_ai_response(mut event_reader: EventReader<AiResponseEvent>) {
    for event in event_reader.read() {
        println!("response: {}", event.0);
    }
}
