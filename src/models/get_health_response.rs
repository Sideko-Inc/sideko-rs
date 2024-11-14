#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetHealthResponse {
    #[cfg_attr(feature = "cli", arg(id = "ok", long = "ok"))]
    pub ok: bool,
}
