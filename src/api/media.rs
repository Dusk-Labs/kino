use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MediaOutput {
    pub id: usize,
    pub library_id: usize,
    pub name: String,
    pub description: String,
    pub rating: usize,
    pub year: usize,
    pub added: String,
    pub poster_path: String,
    pub backdrop_path: String,
    pub media_type: String,
    pub genres: Vec<String>,
    pub duration: usize,
    pub duration_pretty: String,
}

pub async fn get(id: usize, token: &str) -> Result<MediaOutput, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://localhost:8000/api/v1/media/{}", id))
        .header("authorization", token)
        .send()
        .await?;
    let body = res.json::<MediaOutput>().await?;
    Ok(body)
}

#[derive(Deserialize, Debug)]
pub struct MediaInfoOutput {
    pub progress: usize,
    pub versions: Vec<MediaVersionsOutput>,
}

#[derive(Deserialize, Debug)]
pub struct MediaVersionsOutput {
    pub display_name: String,
    pub file: String,
    pub id: usize,
}

pub async fn info(id: usize, token: &str) -> Result<MediaInfoOutput, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://localhost:8000/api/v1/media/{}/info", id))
        .header("authorization", token)
        .send()
        .await?;
    let body = res.json::<MediaInfoOutput>().await?;
    Ok(body)
}
