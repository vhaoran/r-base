use super::Config;
use log::*;
use std::sync::Arc;

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use elasticsearch::cert::CertificateValidation;
use elasticsearch::{
    auth::Credentials,
    cat::CatIndicesParts,
    http::transport::{SingleNodeConnectionPool, Transport, TransportBuilder},
    Elasticsearch, Error, DEFAULT_ADDRESS,
};
use once_cell::sync::OnceCell;
use url::Url;
static INSTANCE: OnceCell<Arc<Elasticsearch>> = OnceCell::new();

pub async fn init(cfg: &Config) -> Result<(), Box<dyn std::error::Error>> {
    debug!("-----res init start----");

    let c = get_client(cfg.url.as_str())?;
    let a = Arc::new(c);
    if let Err(_) = INSTANCE.set(a) {
        error!("redis init error");
        panic!("redis init error");
    }

    debug!("-----res init ok----");

    Ok(())
}

pub fn cnt() -> Arc<Elasticsearch> {
    INSTANCE.get().unwrap().clone()
}

pub fn get_client(path: &str) -> Result<Elasticsearch, Box<dyn std::error::Error>> {
    // let transport = Transport::single_node(path)?;
    // Ok(Elasticsearch::new(transport))

    let url = Url::parse(path)?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
    Ok(Elasticsearch::new(transport))
}

pub fn get_client_bak(path: &str) -> Result<Elasticsearch, Box<dyn std::error::Error>> {
    let mut url = Url::parse(path)?;

    // if the url is https and specifies a username and password, remove from the url and set credentials
    let credentials = if url.scheme() == "https" {
        let username = if !url.username().is_empty() {
            let u = url.username().to_string();
            url.set_username("").unwrap();
            u
        } else {
            "root".to_string()
        };

        let password = match url.password() {
            Some(p) => {
                let pass = p.to_string();
                url.set_password(None).unwrap();
                pass
            }
            None => "password".to_string(),
        };

        Some(Credentials::Basic(username, password))
    } else {
        None
    };

    let conn_pool = SingleNodeConnectionPool::new(url);
    let mut builder = TransportBuilder::new(conn_pool);

    builder = match credentials {
        Some(c) => {
            builder = builder.auth(c);

            #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
            {
                builder = builder.cert_validation(CertificateValidation::None);
            }

            builder
        }
        None => builder,
    };

    let transport = builder.build()?;
    Ok(Elasticsearch::new(transport))
}
