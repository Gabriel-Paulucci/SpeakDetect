use std::{collections::HashMap, sync::Arc};

use serenity::{
    async_trait,
    model::prelude::{Channel, Ready},
    prelude::{Context, EventHandler},
};
use songbird::CoreEvent;
use tokio::sync::Mutex;

use crate::{data::BotData, receiver::Receiver};

pub struct Events;

#[async_trait]
impl EventHandler for Events {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("Bot {} start", data_about_bot.user.name);

        let data = ctx.data.read().await;
        let data = data.get::<BotData>();

        let data = if let Some(data) = data {
            data
        } else {
            return;
        };

        let channel = ctx.http.get_channel(data.channel_id).await;

        let channel = if let Ok(Channel::Guild(channel)) = channel {
            channel
        } else {
            return;
        };

        println!("Start songbird");

        let manager = songbird::get(&ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();

        println!("Connect to channel");

        let (handler_lock, conn_result) = manager.join(channel.guild_id, channel.id).await;

        if conn_result.is_err() {
            println!("{:?}", conn_result.unwrap_err());
            return;
        }

        println!("Connected");

        let mut handler = handler_lock.lock().await;

        let map = Arc::new(Mutex::new(HashMap::new()));

        handler.add_global_event(
            CoreEvent::SpeakingUpdate.into(),
            Receiver {
                notify: data.notify,
                users_ssrc: map.clone(),
            },
        );

        handler.add_global_event(
            CoreEvent::SpeakingStateUpdate.into(),
            Receiver {
                notify: data.notify,
                users_ssrc: map,
            },
        );
    }
}
