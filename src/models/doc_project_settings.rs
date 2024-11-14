#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectSettings {
    #[cfg_attr(feature = "cli", arg(id = "action-button", long = "action-button"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::DocProjectActionButton>
        )
    )]
    pub action_button: crate::models::DocProjectActionButton,
    #[cfg_attr(feature = "cli", arg(id = "metadata", long = "metadata"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::DocProjectMetadata>
        )
    )]
    pub metadata: crate::models::DocProjectMetadata,
}
