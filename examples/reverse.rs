#[tokio::main]
async fn main() {
    let client = nominatim_rs::Client::new(
        reqwest::Url::parse("https://nominatim.openstreetmap.org/").unwrap(),
        concat!(
            env!("CARGO_CRATE_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
            " ",
            "test-suite"
        )
        .to_string(),
        Some("john_t@mailo.com".to_string()),
    )
    .unwrap();

    let reverse = client
        .reverse(
            nominatim_rs::ReverseQueryBuilder::default()
                .address_details(true)
                .lon(13.53918)
                .lat(52.5460941)
                .zoom(nominatim_rs::Zoom::Building)
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{}", serde_json::to_string_pretty(&reverse).unwrap());
}