[中文](https://github.com/hytracen/bevy_openai/blob/master/README_CN.md)
# bevy_openai
`bevy_openai` is an **event-driven** plugin for [Bevy](https://bevyengine.org) that provides **convenient** access to the [OpenAI API](https://beta.openai.com/docs/api-reference/introduction).

Current features:

- [x] [ChatCompletion](https://platform.openai.com/docs/guides/text-generation/chat-completions-api)

## Installation

Add the crate as a dependency:

```toml
[dependencies]
bevy_openai = "0.1.0"
```

Add the plugin:

```rust
use bevy::prelude::*;
use bevy_openai::OpenAiPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, OpenAiPlugin))
}
```
# Usage

Set OPENAI_API_KEY to environment variable 
```bash
$ export OPENAI_API_KEY=sk-xxxxxxx
```
`bevy_openai` is event-driven. You can send a prompt to the ChatGPT and read the response from the ChatGPT using events.

Use `SendToAiEvent` to send your prompt to the ChatGPT.

```rust
fn send_to_ai(
    mut event_writer: EventWriter<SendToAiEvent>
) {
    event_writer.send(SendToAiEvent("Hello".to_string()));
}
```
Or use `SendToAiWithConfigEvent` to send your prompt to the ChatGPT with a custom config.
```rust
fn send_to_ai(
    mut config_event_writer: EventWriter<SendToAiWithConfigEvent>,
) {
    // with config
    config_event_writer.send(SendToAiWithConfigEvent {
        prompt: "Hello".to_string(),
        config: ClientConfigBuilder::default()
            .api_endpoint("".to_owned())
            .api_key("".to_owned())
            .build()
            .expect("Failed to build config"),
    });
}
```
Use `AiResponseEvent` to read the response from the ChatGPT.
```rust
fn process_ai_response(mut event_reader: EventReader<AiResponseEvent>) {
    for event in event_reader.read() {
        println!("response: {}", event.0);
    }
}
```
The full example is in the `examples` directory.
