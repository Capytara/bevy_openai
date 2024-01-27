# bevy_openai

`bevy_openai`是一个由事件驱动的Bevy插件，能够方便的在[Bevy](https://bevyengine.org)中调用[OpenAI API](https://beta.openai.com/docs/api-reference/introduction)。

当前功能：

-[x] [ChatCompletion](https://platform.openai.com/docs/guides/text-generation/chat-completions-api)

# 安装
将crate添加为依赖：
```toml
[dependencies]
bevy_openai = "0.1.0"
```
添加插件：
```rust
use bevy::prelude::*;
use bevy_openai::OpenAiPlugin;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, OpenAiPlugin))
}
```

# 使用
将`OPENAI_API_KEY`设置为环境变量

```bash
$ export OPENAI_API_KEY=sk-xxxxxxx
```
`bevy_openai`是事件驱动的。你可以使用事件向ChatGPT发送prompt，也可以使用事件读取来自ChatGPT的响应。

使用`SendToAiEvent`向ChatGPT发送prompt。
```rust
fn send_to_ai(mut event_writer: EventWriter<SendToAiEvent>) {
    event_writer.send(SendToAiEvent("Hello".to_string()));
}
```
或者使用`SendToAiWithConfigEvent`来添加自定义的配置信息。
```rust
fn send_to_ai(mut config_event_writer: EventWriter<SendToAiWithConfigEvent>, ) {
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
最后，使用`AiResponseEvent`读取来自ChatGPT的响应。

```rust
fn process_ai_response(mut event_reader: EventReader<AiResponseEvent>) {
    for event in event_reader.read() {
        println!("response: {}", event.0);
    }
}
```
完整的示例在`examples`目录中。