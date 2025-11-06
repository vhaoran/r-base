use crate::g::date;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::*;
use tracing::*;

use super::*;

pub async fn publish_json<Q, T>(topic: Q, body: T) -> anyhow::Result<()>
where
    T: Serialize,
    Q: AsRef<str> + std::fmt::Display,
{
    let topic = topic.to_string();
    // let topic = topic.to_string();
    let s = serde_json::to_string(&body)?;
    debug!("--before_mq_publish_{topic} data: {s}-------");
    let _ = self::publish(topic.clone(), s.as_str()).await?;
    debug!("--after_mq_publish_{topic} data: {s}-------");
    Ok(())
}

pub async fn publish<T>(topic: T, body: &str) -> anyhow::Result<()>
where
    T: AsRef<str> + std::fmt::Display,
{
    // debug!("--enter_nats_publish-------");

    let topic = topic.to_string();
    let conn = super::cnt();

    let start = date::now().timestamp_millis();

    let msg = body.to_string();
    let payload = msg.as_bytes().to_vec();
    let _ = conn
        .publish(topic.clone(), Bytes::from(payload))
        .await
        .map_err(|e| {
            error!("---nats_publish_error---{}-", e.to_string());
            e
        })?;
    debug!("--after_mq_publish_{topic} data: {msg}-------");

    // let _ = conn.publish(topic.as_str(), body).map_err(|e| {
    //     error!("---nats_publish_error---{}-", e.to_string());
    //     e
    // })?;

    let offset = date::now().timestamp_millis() - start;
    debug!("nats published : ms: {offset} topic: {topic} body {body}");

    Ok(())
}
