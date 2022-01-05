use axum::http::StatusCode;
use csv::Writer;

pub fn write_csv(path: &str, data: &str) -> Result<String, StatusCode> {
    if let Ok(mut writer) = Writer::from_path(path) {
        println!("Now write...");
        let line_wr: Result<(), csv::Error> = data
            .split("\n")
            .map(|line| writer.write_record(line.split(",")))
            .collect::<Vec<_>>()
            .into_iter()
            .collect();

        println!("line write stat: {:?}", line_wr);
        if let Ok(()) = line_wr {
            return writer
                .flush()
                .and(Ok("Csv Saved".to_owned()))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
