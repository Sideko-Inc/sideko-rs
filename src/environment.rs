use std::fmt::Display;
#[derive(Debug, Default, Clone)]
pub enum Environment {
    #[default]
    Production,
    Staging,
}
impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Production => write!(f, "https://api.sideko.dev/v1"),
            Environment::Staging => write!(f, "https://api.sideko-staging.dev/v1"),
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) enum BaseUrl {
    Env(crate::environment::Environment),
    Custom(String),
}
impl Default for BaseUrl {
    fn default() -> Self {
        BaseUrl::Env(crate::environment::Environment::default())
    }
}
impl std::fmt::Display for BaseUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Env(e) => write!(f, "{e}"),
            Self::Custom(url) => write!(f, "{url}"),
        }
    }
}