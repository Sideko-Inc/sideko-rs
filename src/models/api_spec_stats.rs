#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiSpecStats {
    #[cfg_attr(
        feature = "cli",
        arg(id = "authenticated-methods", long = "authenticated-methods")
    )]
    pub authenticated_methods: i64,
    #[cfg_attr(
        feature = "cli",
        arg(id = "authentication-schemes", long = "authentication-schemes")
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub authentication_schemes: Vec<String>,
    #[cfg_attr(feature = "cli", arg(id = "endpoints", long = "endpoints"))]
    pub endpoints: i64,
    #[cfg_attr(feature = "cli", arg(id = "methods", long = "methods"))]
    pub methods: i64,
    #[cfg_attr(feature = "cli", arg(id = "public-methods", long = "public-methods"))]
    pub public_methods: i64,
    #[cfg_attr(feature = "cli", arg(id = "response-codes", long = "response-codes"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub response_codes: Vec<i64>,
}
