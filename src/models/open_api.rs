#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OpenApi {
    #[cfg_attr(feature = "cli", arg(id = "is-config-valid", long = "is-config-valid"))]
    pub is_config_valid: bool,
    #[cfg_attr(feature = "cli", arg(id = "is-valid", long = "is-valid"))]
    pub is_valid: bool,
    #[cfg_attr(feature = "cli", arg(id = "openapi", long = "openapi"))]
    pub openapi: String,
    #[cfg_attr(feature = "cli", arg(id = "validations", long = "validations"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::Validation>)
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub validations: Vec<crate::models::Validation>,
}
