#[tokio::main]
async fn main() {
    let client = nominatim_rs::Client::new(
        url::Url::parse("https://nominatim.openstreetmap.org/").unwrap(),
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

    let search = client
        .search(
            nominatim_rs::SearchQueryBuilder::default()
                .address_details(true)
                .location_query(nominatim_rs::LocationQuery::Generalised {
                    q: "bakery in berlin wedding".to_string(),
                })
                .limit(Some(1))
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{}", serde_json::to_string_pretty(&search).unwrap());
}
