#[tokio::test]
async fn search() {
    let client = nominatim::Client::new(
        url::Url::parse("https://nominatim.openstreetmap.org/").unwrap(),
        "nominatim-rust/0.1.0 test-suite".to_string(),
        Some("john_t@mailo.com".to_string()),
    )
    .unwrap();

    let search = client
        .search(
            nominatim::search::SearchQueryBuilder::default()
                .address_details(true)
                .location_query(nominatim::search::LocationQuery::Generalised {
                    q: "bakery in berlin wedding".to_string(),
                })
                .limit(Some(1))
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{}", serde_json::to_string(&search).unwrap());
}