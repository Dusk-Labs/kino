use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct LoginInput {
    pub username: String,
    pub password: String,
    pub invite_token: Option<String>,
}

#[derive(Deserialize, Debug)]
struct LoginOutput {
    pub token: String,
}

pub async fn login(username: &str, password: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8000/api/v1/auth/login")
        .json(&LoginInput {
            username: username.to_string(),
            password: password.to_string(),
            invite_token: None,
        })
        .send()
        .await?;
    let body = res.json::<LoginOutput>().await?;
    Ok(body.token)
}

#[derive(Deserialize, Debug)]
struct AdminExistsOutput {
    pub exists: bool,
}

pub async fn admin_exists() -> Result<bool, reqwest::Error> {
    let res = reqwest::get("http://localhost:8000/api/v1/auth/admin_exists").await?;

    assert!(res.status().is_success());
    let body = res.json::<AdminExistsOutput>().await?;
    Ok(body.exists)
}

#[derive(Serialize, Debug)]
struct RegisterInput {
    pub username: String,
    pub password: String,
}

pub async fn register(username: &str, password: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post("http://localhost:8000/api/v1/auth/register")
        .json(&RegisterInput {
            username: username.to_string(),
            password: password.to_string(),
        })
        .send()
        .await?;
    Ok(())
}
