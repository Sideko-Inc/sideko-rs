#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PinnedApiLinkPolicyTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "pinned"))]
    #[serde(rename = "pinned")]
    Pinned,
}
impl std::fmt::Display for PinnedApiLinkPolicyTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PinnedApiLinkPolicyTypeEnum::Pinned => "pinned",
        };
        write!(f, "{}", str_val)
    }
}
