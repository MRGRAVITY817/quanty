use reqwest::{Client, StatusCode};
use select::{document::Document, node::Node};

pub fn node_to_text(node: Node) -> String {
    node.text().trim().to_string()
}

// TODO: Error type is not correct.
pub async fn read_html(url: &str) -> Result<Document, StatusCode> {
    if let Ok(res) = reqwest::get(url).await {
        if let Ok(ref text) = res.text().await {
            return Document::from_read(&mut text.as_bytes()).map_err(|_| StatusCode::NOT_FOUND);
        }
    }
    Err(StatusCode::NOT_FOUND)
}

pub async fn read_raw_html(url: &str) -> Result<String, StatusCode> {
    if let Ok(res) = reqwest::get(url).await {
        if let Ok(ref text) = res.text().await {
            return Ok(text.into());
        }
    }
    Err(StatusCode::NOT_FOUND)
}

pub async fn read_post_html(
    url: &str,
    params: &[(&str, &str)],
    client: Client,
) -> Result<Document, StatusCode> {
    if let Ok(res) = client.post(url).form(params).send().await {
        if let Ok(ref text) = res.text().await {
            return Document::from_read(&mut text.as_bytes()).map_err(|_| StatusCode::NOT_FOUND);
        }
    }
    Err(StatusCode::NOT_FOUND)
}
