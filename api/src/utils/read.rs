use axum::http::HeaderMap;
use reqwest::{Client, StatusCode};
use select::{
    document::Document,
    node::{Find, Node},
    predicate::Name,
};

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

pub async fn read_post_html(
    url: &str,
    params: &[(&str, &str)],
    headers: HeaderMap,
    client: &Client,
) -> Result<Document, StatusCode> {
    if let Ok(res) = client.post(url).form(params).headers(headers).send().await {
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

pub async fn read_post_raw(
    url: &str,
    params: &[(&str, &str)],
    headers: HeaderMap,
    client: &Client,
) -> Result<String, StatusCode> {
    if let Ok(res) = client.post(url).form(params).headers(headers).send().await {
        if let Ok(ref text) = res.text().await {
            return Ok(text.into());
        }
    }
    Err(StatusCode::NOT_FOUND)
}

// Find node element for given xpath in document
// pub fn xpath_to_node(xpath: &str, doc: Document) -> Option<Node> {
//     let xpath = "/html/body/div[3]/div[2]/div[2]/div[1]/div[2]/div[1]/div/ul[2]/li/span";
//     let xpath_from_body = &xpath.split("/").collect::<Vec<&str>>()[2..];

//     for node in xpath_from_body {
//         let k = node
//     }

//     let get_node_from_path = |node: Node, path: &str| {
//         let name_index = path.split(&['[', ']'][..]);
//         if let Some(name) = name_index.nth(0) {
//             if let Some(str_index) = name_index.nth(1) {
//                 if let Ok(index) = str_index.parse::<usize>() {
//                     node.find(Name(name)).nth(index-1)
//                 } else {
//                     None
//                 }
//             } else {
//                     node.find(Name(name)).nth(0)
//             }
//         } else {
//             None
//         }
//     };

//     //TODO: Iterate xpath.split("/")
//     let html = doc.find(Name("html")).nth(0).unwrap();
//     let body = get_node_from_path(html, "body");
//     let k = xpath.split("/").next().map(|path| {
//     });

//     let node_by_name = |name: &str, found_node: Find| found_node.find(Name(name));
//     xpath.split()

//     let node = doc.find(Name("html"));
//     //TODO: prinln!() to print out the result to check if we are doing fine
// }
