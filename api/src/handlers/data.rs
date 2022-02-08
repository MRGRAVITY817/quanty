use axum::response::IntoResponse;
use reqwest::StatusCode;

use crate::utils::{read::read_raw_html, time::get_date_yearly};

/// Get chart data from Naver finance
pub async fn get_chart_data() -> impl IntoResponse {
    let base_url = "https://fchart.stock.naver.com/siseJson.nhn";
    let symbol = "005930";
    let from = get_date_yearly(5);
    let to = get_date_yearly(2);

    let url = format!(
        "{base_url}?symbol={symbol}&requestType=1&startTime={from}&endTime={to}&timeframe=day"
    );

    match read_raw_html(&url).await {
        Ok(raw_table) => {
            let table = raw_table
                .replace('\n', "")
                .replace(" ", "")
                .trim_end()
                .trim_start()
                .split("],")
                .map(|row| row.trim().replace("[", ""))
                .collect::<Vec<_>>();

            // header: '날짜','시가','고가','저가','종가','거래량','외국인소진율'
            // we only need '날짜' and '종가', which is column 0 and 4.
            let body = table[1..].into_iter().map(|row| {
                let mut columns = row.split(',');
                let date = columns.nth(0).unwrap_or("");
                let close = columns.nth(4).unwrap_or("");
                (date, close)
            });

            // write csv
            let wtr = csv::Writer::from_path("date_close.csv");
            match wtr {
                Ok(mut w) => {
                    let write_header = w.write_record(&["Date(Year/Month/Day)", "Close Price"]);
                    match write_header {
                        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                        _ => {}
                    }
                    for (date, close) in body {
                        match w.write_record(&[date, close]) {
                            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                            _ => {}
                        }
                    }
                    Ok("CSV successfully written!".to_owned())
                }
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
