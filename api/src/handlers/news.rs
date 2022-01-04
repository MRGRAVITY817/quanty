use reqwest::StatusCode;
use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};

const NEWS_URL: &'static str =
    "https://finance.naver.com/news/news_list.nhn?mode=LSS2D&section_id=101&section_id2=258";

/// Get news from Naver Finance
pub async fn get_news() -> Result<String, StatusCode> {
    if let Ok(res) = reqwest::get(NEWS_URL).await {
        if let Ok(ref text) = res.text().await {
            Document::from_read(&mut text.as_bytes())
                .and_then(|doc| {
                    Ok(doc
                        .find(
                            Name("dl")
                                .descendant(Class("articleSubject"))
                                .descendant(Name("a")),
                        )
                        .filter_map(|node| node.attr("title"))
                        .collect::<Vec<_>>()
                        .join("\n"))
                })
                .map_err(|_| StatusCode::NOT_FOUND)
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
