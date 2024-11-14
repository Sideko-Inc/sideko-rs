#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Theme {
    #[cfg_attr(feature = "cli", arg(id = "owner", long = "owner"))]
    pub owner: crate::models::ThemeOwnerEnum,
    #[cfg_attr(feature = "cli", arg(id = "values", long = "values"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::ThemeValues>)
    )]
    pub values: crate::models::ThemeValues,
}
