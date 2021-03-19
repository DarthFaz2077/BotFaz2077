use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub discord_token: String,
    pub prefix: String,
    pub activity: String,
}
