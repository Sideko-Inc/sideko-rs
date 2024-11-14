#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ThemeValues {
    #[cfg_attr(
        feature = "cli",
        arg(id = "api-reference-group-variant", long = "api-reference-group-variant")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_reference_group_variant: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "dark-active-button-bg-color", long = "dark-active-button-bg-color")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_active_button_bg_color: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "dark-active-button-text-color", long = "dark-active-button-text-color")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_active_button_text_color: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "dark-bg-color", long = "dark-bg-color"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_bg_color: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "dark-navbar-color", long = "dark-navbar-color")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_navbar_color: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "dark-navbar-text-color", long = "dark-navbar-text-color")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_navbar_text_color: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "light-active-button-bg-color", long = "light-active-button-bg-color")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_active_button_bg_color: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(
            id = "light-active-button-text-color",
            long = "light-active-button-text-color"
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_active_button_text_color: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "light-bg-color", long = "light-bg-color"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_bg_color: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "light-navbar-color", long = "light-navbar-color")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_navbar_color: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "light-navbar-text-color", long = "light-navbar-text-color")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_navbar_text_color: Option<String>,
}
