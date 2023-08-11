use serenity::{framework::StandardFramework, prelude::GatewayIntents, Client};
use songbird::{driver::DecodeMode, Config, SerenityInit};

use crate::{commands, data::BotData, event::Events, Notify};

#[tokio::main]
pub async fn startup_bot(token: &str, channel_id: u64, notify: Notify) -> Result<(), String> {
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|x| x.prefix("lv-"))
        .group(&commands::GENERAL_GROUP);

    let songbird_config = Config::default().decode_mode(DecodeMode::Decode);

    let mut client = Client::builder(token, intents)
        .event_handler(Events)
        .framework(framework)
        .register_songbird_from_config(songbird_config)
        .await
        .expect("Fail to start bot");

    {
        let mut data = client.data.write().await;

        data.insert::<BotData>(BotData { channel_id, notify });
    }

    client.start().await.map_err(|why| why.to_string())
}
