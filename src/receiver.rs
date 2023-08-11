use std::{collections::HashMap, sync::Arc};

use serenity::async_trait;
use songbird::{model::payload::Speaking, EventContext, EventHandler};
use tokio::sync::Mutex;

use crate::Notify;

pub struct Receiver {
    pub notify: Notify,
    pub users_ssrc: Arc<Mutex<HashMap<u32, u64>>>,
}

#[async_trait]
impl EventHandler for Receiver {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<songbird::Event> {
        match ctx {
            EventContext::SpeakingStateUpdate(Speaking { ssrc, user_id, .. }) => {
                println!("Update {:?}", user_id);
                if user_id.is_none() {
                    return None;
                }

                let user_id = user_id.unwrap().0;

                let mut map = self.users_ssrc.lock().await;

                *map.entry(*ssrc).or_insert(user_id) = user_id;
            }
            EventContext::SpeakingUpdate(data) => {
                let map = self.users_ssrc.lock().await;

                let user_id = map.get(&data.ssrc);

                println!("Speak: {:?}, {}", user_id, data.speaking);

                if let Some(user_id) = user_id {
                    unsafe { (self.notify)(*user_id, data.speaking) };
                }
            }
            _ => {}
        };

        None
    }
}
