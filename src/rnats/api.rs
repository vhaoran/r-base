use crate::g::date;
use serde::{Deserialize, Serialize};
use serde_json::*;
use tracing::*;

use super::*;

pub fn publish_json<Q, T>(topic: Q, body: T) -> anyhow::Result<()>
where
    T: Serialize,
    Q: AsRef<str> + std::fmt::Display,
{
    // let topic = topic.to_string();
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
                data
            });
    });

    Ok(())
}

pub fn publish<T>(topic: T, body: &str) -> anyhow::Result<()>
where
    T: AsRef<str> + std::fmt::Display,
{
    // debug!("--enter_nats_publish-------");

    let topic = topic.to_string();
    let conn = super::cnt();

    let start = date::now().timestamp_millis();

    let _ = conn.publish(topic.as_str(), body).map_err(|e| {
        error!("---nats_publish_error---{}-", e.to_string());
        e
    })?;

    let offset = date::now().timestamp_millis() - start;
    debug!("nats published : ms: {offset} topic: {topic} body {body}");

    Ok(())
}
