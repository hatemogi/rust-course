use crate::url;
use async_trait::async_trait;
use hex;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[async_trait]
pub trait UrlStore {
    /* 단축URL에서 (다이제스트, 긴URL) 조회 */
    async fn find_by_short(&self, short_url: &str) -> Option<(String, String)>;

    /* URL키 기준에서 짧은URL 조회 */
    async fn find_by_digest(&self, digest: &Vec<u8>) -> Option<String>;

    /* 저장하기 */
    async fn save(&mut self, short_url: &str, digest: &Vec<u8>, long_url: &str);
}

pub struct MemoryUrlStore {
    db: HashMap<String, (String, String)>,
    digests: HashMap<String, String>,
}

impl MemoryUrlStore {
    fn new() -> MemoryUrlStore {
        let db = HashMap::new();
        let digests = HashMap::new();
        MemoryUrlStore { db, digests }
    }
}

#[async_trait]
impl UrlStore for MemoryUrlStore {
    async fn find_by_short(&self, short_url: &str) -> Option<(String, String)> {
        self.db.get(short_url).cloned()
    }

    async fn find_by_digest(&self, digest: &Vec<u8>) -> Option<String> {
        self.digests.get(&hex::encode(digest)).cloned()
    }

    async fn save(&mut self, short_url: &str, digest: &Vec<u8>, long_url: &str) {
        let digest_hex = hex::encode(digest);
        self.db.insert(
            String::from(short_url),
            (digest_hex.clone(), String::from(long_url)),
        );
        self.digests.insert(digest_hex, String::from(short_url));
    }
}

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::types::AttributeValue::{N, S};
use aws_sdk_dynamodb::Client;

pub struct DynamoUrlStore {
    client: Client,
}

impl DynamoUrlStore {
    pub async fn new() -> DynamoUrlStore {
        let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-2");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        DynamoUrlStore { client }
    }
}

const TABLE_NAME: &str = "shorten.url";
const INDEX_NAME: &str = "shorten.url.digest-index";

#[async_trait]
impl UrlStore for DynamoUrlStore {
    async fn find_by_short(&self, short_url: &str) -> Option<(String, String)> {
        let request = self
            .client
            .get_item()
            .table_name(TABLE_NAME)
            .key("short_url", S(short_url.to_owned()));
        let result = request.send().await;
        result.ok().and_then(|output| {
            println!("output = {:?}", output);
            let attrs = output.item()?;
            let digest = attrs.get("digest")?.as_s().ok()?;
            let long_url = attrs.get("long_url")?.as_s().ok()?;
            Some((digest.to_owned(), long_url.to_owned()))
        })
    }

    async fn find_by_digest(&self, digest: &Vec<u8>) -> Option<String> {
        let request = self
            .client
            .get_item()
            .table_name(INDEX_NAME)
            .key("digest", S(hex::encode(digest)));
        let result = request.send().await;
        result.ok().and_then(|output| {
            let item = output.item()?.get("short_url")?.as_s().ok()?;
            Some(item.to_owned())
        })
    }

    async fn save(&mut self, short_url: &str, digest: &Vec<u8>, long_url: &str) {
        let timestamp: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let request = self
            .client
            .put_item()
            .table_name(TABLE_NAME)
            .item("short_url", S(short_url.to_owned()))
            .item("digest", S(hex::encode(digest)))
            .item("long_url", S(long_url.to_owned()))
            .item("created_at", N(timestamp.to_string()));
        request.send().await.unwrap();
    }
}

pub async fn shorten(store: &mut impl UrlStore, long_url: &str) -> String {
    let long_url = long_url.trim();
    let digest = url::digest(long_url);
    let digest_hex = hex::encode(&digest);
    for t in 2..digest.len() {
        let short_url = url::truncated_base32(&digest, t);
        match store.find_by_short(&short_url).await {
            Some((_, hex)) if hex == digest_hex => return short_url,
            _ => {
                let _ = store.save(&short_url, &digest, long_url).await;
                return short_url;
            }
        }
    }
    panic!("endless duplication")
}

pub async fn lengthen(store: &impl UrlStore, short_url: &str) -> Option<String> {
    let normalized_url = url::base32_normalize(short_url)?;
    let (_digest, url) = store.find_by_short(&normalized_url).await?;
    Some(String::from(url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mem_store() {
        let mut store = MemoryUrlStore::new();
        assert_eq!(true, store.find_by_short("short-url-1").await.is_none());
        let digest = url::digest("long-url-1");
        let digest_hex = hex::encode(&digest);
        store.save("short-url-1", &digest, "long-url-1").await;
        assert_eq!(
            Some((digest_hex, String::from("long-url-1"))),
            store.find_by_short("short-url-1").await
        );
    }

    #[tokio::test]
    async fn test_shorten() {
        let mut store = MemoryUrlStore::new();
        let long_url = "a-long-url";
        let short_url = shorten(&mut store, long_url).await;
        // println!("short_url = {}", short_url);
        assert_eq!(true, store.find_by_short(&short_url).await.is_some());
    }

    #[tokio::test]
    async fn test_lengthen() {
        let mut store = MemoryUrlStore::new();
        let long_url = "a-long-url";
        let short_url = shorten(&mut store, long_url).await;
        // println!("short_url = {}", short_url);
        let lengthen_url = lengthen(&store, &short_url).await;
        assert_eq!(Some(String::from(long_url)), lengthen_url);
        assert_eq!(true, lengthen(&store, "unknown-url").await.is_none());
    }
}
