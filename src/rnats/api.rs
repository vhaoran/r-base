use super::*;
use log::*;

use serde::{Deserialize, Serialize};
use serde_json::*;

pub fn publish_json<T>(topic: &str, body: T) -> anyhow::Result<()>
where
    T: Serialize,
{
    let s = serde_json::to_string(&body)?;

    let _ = self::publish(topic, s.as_str())?;
    Ok(())
}

pub async fn publish_json_async<T, TP>(topic: TP, body: T) -> anyhow::Result<()>
where
    T: Serialize,
    TP: AsRef<str> + std::fmt::Display,
{
    let topic = topic.to_string();

    let s = serde_json::to_string(&body)?;
    // let topic = topic.clone();
    tokio::spawn(async move {
        let _ = self::publish(topic.as_str(), s.as_str())
            .map_err(|e| {
                error!("--publish msg--error---{}-", e.to_string());
            })
            .map(|data| {
                debug!("--publish msg-ok ---{:?}---", s);
            });
    });

    Ok(())
}

pub fn publish<T>(topic: T, body: &str) -> anyhow::Result<()>
where
    T: AsRef<str> + std::fmt::Display,
{
    debug!("---start publish------");
    let topic = topic.to_string();

    let conn = super::cnt();

    debug!("topic: {}  publish : {}", topic, body);
    let _ = conn.publish(topic.as_str(), body)?;

    Ok(())
}
