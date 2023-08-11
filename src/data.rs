use serenity::prelude::TypeMapKey;

use crate::Notify;

pub struct BotData {
    pub channel_id: u64,
    pub notify: Notify,
}

impl TypeMapKey for BotData {
    type Value = BotData;
}
