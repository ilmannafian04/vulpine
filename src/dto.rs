use serde::Deserialize;

#[derive(Deserialize)]
pub struct SleepArgs {
    pub duration: Option<u64>,
    #[serde(default)]
    pub random: bool,
    pub min_duration: Option<u64>,
    pub max_duration: Option<u64>,
}

#[derive(Deserialize)]
pub struct ThrowawayParam {
    pub value: Option<String>,
}
