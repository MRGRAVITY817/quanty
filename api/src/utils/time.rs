use chrono::prelude::*;

pub fn get_date_yearly(year_ago: i32) -> String {
    let now = Local::now();
    Utc.ymd(now.year() - year_ago, now.month(), now.day())
        .to_string()
        .replace("-", "")
        .replace("UTC", "")
}
