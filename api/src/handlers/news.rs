use crate::utils::{node_to_text, read_html, read_post_html};
use reqwest::StatusCode;
use select::predicate::{Class, Name, Predicate};

const NEWS_URL: &'static str =
    "https://finance.naver.com/news/news_list.nhn?mode=LSS2D&section_id=101&section_id2=258";
const DISCLOSURE_URL: &'static str = "https://kind.krx.co.kr/disclosure/todaydisclosure.do";

/// Get news from Naver Finance
pub async fn get_financial_news() -> Result<String, StatusCode> {
    let html = read_html(NEWS_URL).await;
    html.map(|doc| {
        doc.find(
            Name("dl")
                .descendant(Class("articleSubject"))
                .descendant(Name("a")),
        )
        .filter_map(|a| a.attr("title"))
        .collect::<Vec<_>>()
        .join("\n")
    })
}

/// Get today's disclosure from kind.krx
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
    let html = read_post_html(DISCLOSURE_URL, &params, client).await;
    html.map(|doc| {
        doc.find(Name("table").descendant(Name("tr")))
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
            .join("\n")
    })
}

const KOSPI_MARKET_SUM_URL: &'static str =
    "https://finance.naver.com/sise/sise_market_sum.nhn?sosok=0&page=1";
const KOSDAQ_MARKET_SUM_URL: &'static str =
    "https://finance.naver.com/sise/sise_market_sum.nhn?sosok=1&page=1";

pub async fn get_ticker() -> Result<String, StatusCode> {
    let html = read_html(KOSPI_MARKET_SUM_URL).await;
    html.map(|doc| {
        doc.find(Class("pgRR").descendant(Name("a")))
            .filter_map(|a| a.attr("href"))
            .collect::<Vec<_>>()
            .join("\n")
    })
}
