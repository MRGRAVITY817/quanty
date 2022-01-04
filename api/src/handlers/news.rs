use reqwest::StatusCode;
use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};

use crate::utils::node_to_text;

const NEWS_URL: &'static str =
    "https://finance.naver.com/news/news_list.nhn?mode=LSS2D&section_id=101&section_id2=258";
const DISCLOSURE_URL: &'static str = "https://kind.krx.co.kr/disclosure/todaydisclosure.do";

// TODO: Refactor this to be less nested
/// Get news from Naver Finance
pub async fn get_financial_news() -> Result<String, StatusCode> {
    if let Ok(res) = reqwest::get(NEWS_URL).await {
        if let Ok(ref text) = res.text().await {
            return Document::from_read(&mut text.as_bytes())
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
                .map_err(|_| StatusCode::NOT_FOUND);
        }
    }
    Err(StatusCode::NOT_FOUND)
}

pub async fn get_today_disclosure() -> Result<String, StatusCode> {
    let client = reqwest::Client::new();
    let params = [
        ("method", "searchTodayDisclosureSub"),
        ("currentPageSize", "15"),
        ("pageIndex", "1"),
        ("orderMode", "0"),
        ("orderStat", "D"),
        ("forward", "todaydisclosure_sub"),
        ("chose", "S"),
        ("todayFlag", "Y"),
        ("selDate", "2021-12-13"),
    ];
    if let Ok(res) = client.post(DISCLOSURE_URL).form(&params).send().await {
        if let Ok(ref text) = res.text().await {
            return Document::from_read(&mut text.as_bytes())
                .and_then(|doc| {
                    Ok(doc
                        .find(Name("table").descendant(Name("tr")))
                        // render table row by row
                        .map(|node| {
                            node.find(Name("td"))
                                .map(|td| {
                                    let table_data = node_to_text(td);
                                    let spans = td
                                        .find(Name("span"))
                                        .map(node_to_text)
                                        .collect::<Vec<_>>()
                                        .join("/");
                                    if spans.len() > 0 {
                                        spans
                                    } else {
                                        table_data
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join("  |  ")
                        })
                        .collect::<Vec<_>>()
                        .join("\n"))
                })
                .map_err(|_| StatusCode::NOT_FOUND);
            // return Ok(text.into());
        }
    }
    Err(StatusCode::NOT_FOUND)
}
