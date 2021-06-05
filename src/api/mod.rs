use serde::Deserialize;

pub mod auth;
pub mod media;

pub type SearchOutput = Vec<SearchOutputItem>;

#[derive(Deserialize, Debug)]
pub struct SearchOutputItem {
    pub name: String,
    pub id: usize,
    pub library_id: usize,
}

pub async fn search(query: &str, token: &str) -> Result<SearchOutput, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:8000/api/v1/search")
        .header("authorization", token)
        .query(&[("query", query)])
        .send()
        .await?;
    let body = res.json::<SearchOutput>().await?;
    Ok(body)
}
