use crate::utils::{read::read_raw_html, time::get_date_yearly};
use axum::response::IntoResponse;

/// Get chart data from Naver finance
pub async fn get_chart_data() -> impl IntoResponse {
    let base_url = "https://fchart.stock.naver.com/siseJson.nhn";
    let symbol = "005930";
    let from = get_date_yearly(5);
    let to = get_date_yearly(2);

    let url = format!(
        "{base_url}?symbol={symbol}&requestType=1&startTime={from}&endTime={to}&timeframe=day"
    );
    println!("{}", &url);

    read_raw_html(&url).await
}
