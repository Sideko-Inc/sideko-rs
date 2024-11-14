#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectSettings {
    #[cfg_attr(feature = "cli", arg(id = "action-button", long = "action-button"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::UpdateDocProjectSettingsActionButton>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_button: Option<crate::models::UpdateDocProjectSettingsActionButton>,
    #[cfg_attr(feature = "cli", arg(id = "metadata", long = "metadata"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::UpdateDocProjectSettingsMetadata>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::UpdateDocProjectSettingsMetadata>,
}
