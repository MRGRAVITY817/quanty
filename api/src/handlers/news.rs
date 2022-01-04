use crate::utils::{node_to_text, read_html, read_post_html, read_post_raw};
use axum::http::{HeaderMap, HeaderValue};
use itertools::Itertools;
use reqwest::{header::REFERER, StatusCode};
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
    let headers = HeaderMap::new();
    let html = read_post_html(DISCLOSURE_URL, &params, headers, &client).await;
    html.map(|doc| {
        if let Some(table) = doc.find(Name("table")).next() {
            let table_header = table
                .find(Name("th"))
                .map(node_to_text)
                .collect::<Vec<_>>()
                .join(" | ");

            let table_rows = table
                .find(Name("tr"))
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
                        .join(" | ")
                })
                .collect::<Vec<_>>()
                .join("\n");

            format!("{}\n{}", table_header, table_rows)
        } else {
            String::from("TABLE NOT FOUND")
        }
    })
}

// Get market sum data from Naver finance
pub async fn get_ticker() -> Result<String, StatusCode> {
    let url = |category: u8, page: u32| {
        format!(
            "https://finance.naver.com/sise/sise_market_sum.nhn?sosok={}&page={}",
            category, page
        )
    };

    let html = read_html(&url(0, 1)).await;
    html.map(|doc| {
        if let Some(table) = doc.find(Name("table")).nth(1) {
            let table_header_list = table.find(Name("th"));
            let cols = table_header_list.count();
            let table_header = format!(
                "{} | {}",
                table
                    .find(Name("th"))
                    .take(cols - 1) // The last column is useless
                    .map(node_to_text)
                    .collect::<Vec<_>>()
                    .join(" | "),
                "종목코드"
            );

            let mut codes = table
                .find(Name("tbody").descendant(Name("td")).descendant(Name("a")))
                .filter_map(|a| a.attr("href").and_then(|href| href.split("code=").last()))
                .unique();

            let table_rows = table
                .find(Name("tr"))
                .map(|tr| {
                    tr.find(Name("td"))
                        .take(cols - 1) // The last column is useless
                        .map(|node| node_to_text(node).trim_matches('\n').to_string())
                        .collect::<Vec<_>>()
                        .join(" | ")
                })
                .filter(|tr| tr.len() > 0)
                .map(|tr| format!("{} | {}", tr, codes.next().unwrap_or("NA")))
                .collect::<Vec<_>>()
                .join("\n");

            format!("{}\n{}", table_header, table_rows)
        } else {
            String::from("TABLE NOT FOUND")
        }
    })
}

const GENERATE_URL: &'static str = "http://data.krx.co.kr/comm/fileDn/GenerateOTP/generate.cmd";
const DOWNLOAD_URL: &'static str = "http://data.krx.co.kr/comm/fileDn/download_csv/download.cmd";
pub async fn get_industry() -> Result<String, StatusCode> {
    let client = reqwest::Client::new();
    let params = [
        ("mktId", "STK"),
        ("trdDd", "20220103"),
        ("money", "1"),
        ("csvxls_isNo", "false"),
        ("name", "fileDown"),
        ("url", "dbms/MDC/STAT/standard/MDCSTAT03901"),
    ];
    let mut headers = HeaderMap::new();
    headers.insert(REFERER, HeaderValue::from_static(GENERATE_URL));
    let download_code = read_post_raw(GENERATE_URL, &params, HeaderMap::new(), &client).await?;
    read_post_raw(DOWNLOAD_URL, &[("code", &download_code)], headers, &client).await
}
