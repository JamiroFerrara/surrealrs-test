pub async fn surreal_post(query: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8000/sql")
        .basic_auth("root", Some("root"))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("NS", "test")
        .header("DB", "test")
        .body(query)
        .send()
        .await?;

    Ok(response)
}

pub async fn surreal_get(query: String) -> Result<serde_json::Value, reqwest::Error> {
    let res = surreal_post(query).await?
            .json::<serde_json::Value>().await?[0]
            .get("result").unwrap().clone();

    Ok(res)
}
