use clap::{CommandFactory, Parser};
use std::str::FromStr;
#[tokio::main]
async fn main() {
    let cli = SidekoCli::parse();
    if let Err(e) = handle_cli(cli).await {
        display_error(e);
        std::process::exit(1);
    }
}
async fn handle_cli(cli: SidekoCli) -> sideko_rest_api::SdkResult<()> {
    init_logger(cli.verbose);
    load_dotsideko();
    let base_url_env_var = "SIDEKO_REST_API_BASE_URL";
    match cli.command {
        SidekoCommand::SidekoConfigSubcommand(
            SidekoConfigSubcommand::Docs { output },
        ) => {
            let markdown = clap_markdown::help_markdown::<SidekoCli>();
            std::fs::write(&output, markdown)?;
            log::info!("CLI documentation saved to {output}")
        }
        SidekoCommand::SidekoConfigSubcommand(
            SidekoConfigSubcommand::Completions { shell },
        ) => {
            let mut cmd = SidekoCli::command();
            let cmd_name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, cmd_name, &mut std::io::stdout());
        }
        SidekoCommand::SidekoConfigSubcommand(
            SidekoConfigSubcommand::BaseUrl { unset, url },
        ) => {
            if unset {
                write_dotsideko(base_url_env_var, "", true);
            } else if let Some(url) = url {
                write_dotsideko(base_url_env_var, &url, false);
            } else {
                log::error!("No base url provided");
                std::process::exit(1);
            }
            log::info!("Base URL updated")
        }
        SidekoCommand::ApiSubcommand(ApiSubcommand::Delete(req)) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            client.api().delete(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::ApiSubcommand(
            ApiSubcommand::ApiSpecSubcommand(ApiSpecSubcommand::Delete(req)),
        ) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            client.api().spec().delete(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::DeleteApiLink(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            client.delete_api_link(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::DeleteApiLinkGroup(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            client.delete_api_link_group(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::DeleteDocProject(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            client.delete_doc_project(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::DeleteGuide(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            client.delete_guide(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::DeleteGuideHref(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            client.delete_guide_href(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::DeleteAsset(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            client.delete_asset(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::RoleSubcommand(RoleSubcommand::Delete(req)) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            client.role().delete(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::DeleteServiceAccount(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            client.delete_service_account(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::ApiSubcommand(ApiSubcommand::List) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.api().list().await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ApiSubcommand(ApiSubcommand::Get(req)) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.api().get(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ApiSubcommand(
            ApiSubcommand::ApiSpecSubcommand(ApiSpecSubcommand::List(req)),
        ) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.api().spec().list(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ApiSubcommand(
            ApiSubcommand::ApiSpecSubcommand(ApiSpecSubcommand::Get(req)),
        ) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.api().spec().get(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ApiSubcommand(
            ApiSubcommand::ApiSpecSubcommand(
                ApiSpecSubcommand::ApiSpecOpenapiSubcommand(
                    ApiSpecOpenapiSubcommand::GetOpenapi(req),
                ),
            ),
        ) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.api().spec().openapi().get_openapi(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ApiSubcommand(
            ApiSubcommand::ApiSpecSubcommand(
                ApiSpecSubcommand::ApiSpecStatsSubcommand(
                    ApiSpecStatsSubcommand::GetStats(req),
                ),
            ),
        ) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.api().spec().stats().get_stats(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ListApiLinks(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.list_api_links(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetApiLink(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_api_link(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ListApiLinkGroups(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.list_api_link_groups(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ExchangeCodeForKey(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            let res = client.exchange_code_for_key(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::LoginCallback(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            client.login_callback(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::LoginUrl(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            client.login_url(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::CliCheckUpdates(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            let res = client.cli_check_updates(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ListDocProjects => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.list_doc_projects().await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetDocProject(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_doc_project(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ListDeployments(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.list_deployments(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetDeployment(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_deployment(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetDocProjectTheme(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_doc_project_theme(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ListDocVersions(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.list_doc_versions(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetDocVersion(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_doc_version(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ListGuides(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.list_guides(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetGuide(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_guide(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetGuideContent(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_guide_content(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetOrganization => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_organization().await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ListAssets(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.list_assets(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetOrganizationTheme => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_organization_theme().await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::RoleSubcommand(RoleSubcommand::List(req)) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.role().list(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::SdkSubcommand(SdkSubcommand::List(req)) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.sdk().list(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetCurrentUser => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_current_user().await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetApiKey => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_api_key().await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ListServiceAccounts => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.list_service_accounts().await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::GetServiceAccount(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.get_service_account(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ApiSubcommand(ApiSubcommand::Patch(req)) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.api().patch(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ApiSubcommand(
            ApiSubcommand::ApiSpecSubcommand(ApiSpecSubcommand::Patch(req)),
        ) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.api().spec().patch(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::UpdateApiLink(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.update_api_link(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::UpdateApiLinkGroup(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.update_api_link_group(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::UpdateDocProject(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.update_doc_project(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::UpdateGuide(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.update_guide(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::UpdateAsset(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.update_asset(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ApiSubcommand(ApiSubcommand::Create(req)) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.api().create(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ApiSubcommand(
            ApiSubcommand::ApiSpecSubcommand(ApiSpecSubcommand::Create(req)),
        ) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.api().spec().create(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::CreateApiLink(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.create_api_link(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ReorderApiLinks(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.reorder_api_links(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::CreateApiLinkGroup(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.create_api_link_group(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::CreateDocProject(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.create_doc_project(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::TriggerDeployment(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.trigger_deployment(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::CreateGuide(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.create_guide(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::ReorderGuides(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.reorder_guides(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::CreateOrganization(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.create_organization(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::UploadAssets(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.upload_assets(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::RoleSubcommand(RoleSubcommand::Create(req)) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.role().create(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::SdkSubcommand(SdkSubcommand::Generate(req)) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.sdk().generate(req).await?;
            save_binary_response(res)?;
        }
        SidekoCommand::SdkSubcommand(
            SdkSubcommand::SdkConfigSubcommand(SdkConfigSubcommand::Init(req)),
        ) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.sdk().config().init(req).await?;
            save_binary_response(res)?;
        }
        SidekoCommand::SdkSubcommand(
            SdkSubcommand::SdkConfigSubcommand(SdkConfigSubcommand::Sync(req)),
        ) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.sdk().config().sync(req).await?;
            save_binary_response(res)?;
        }
        SidekoCommand::SdkSubcommand(SdkSubcommand::Update(req)) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.sdk().update(req).await?;
            println!("{res}");
        }
        SidekoCommand::SdkSubcommand(SdkSubcommand::GenerateStateless(req)) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.sdk().generate_stateless(req).await?;
            save_binary_response(res)?;
        }
        SidekoCommand::InviteUser(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            client.invite_user(req).await?;
            log::info!("API returned no content");
        }
        SidekoCommand::CreateServiceAccount(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.create_service_account(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::UpdateDocProjectTheme(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.update_doc_project_theme(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::UpdateOrganizationTheme(req) => {
            let mut client = sideko_rest_api::Client::default();
            if let Ok(base_url) = std::env::var(base_url_env_var) {
                client = client.with_base_url(&base_url);
                log::debug!("Using custom base url: {base_url}");
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_API_KEY_AUTH") {
                log::debug!("Adding api-key auth 'ApiKeyAuth' (key=\"****\")");
                client = client.with_api_key_auth(&val);
            }
            if let Ok(val) = std::env::var("SIDEKO_REST_API_COOKIE_AUTH") {
                log::debug!("Adding api-key auth 'CookieAuth' (key=\"****\")");
                client = client.with_cookie_auth(&val);
            }
            let res = client.update_organization_theme(req).await?;
            println!(
                "{}", serde_json::to_string_pretty(& res).unwrap_or_else(| _ |
                serde_json::json!(& res) .to_string())
            );
        }
        SidekoCommand::SidekoConfigSubcommand(
            SidekoConfigSubcommand::SidekoAuthSubcommand(
                SidekoAuthSubcommand::ApiKeyAuth { key },
            ),
        ) => {
            write_dotsideko("SIDEKO_REST_API_API_KEY_AUTH", &key, false);
            log::info!("Authentication added to CLI");
        }
        SidekoCommand::SidekoConfigSubcommand(
            SidekoConfigSubcommand::SidekoAuthSubcommand(
                SidekoAuthSubcommand::CookieAuth { key },
            ),
        ) => {
            write_dotsideko("SIDEKO_REST_API_COOKIE_AUTH", &key, false);
            log::info!("Authentication added to CLI");
        }
    }
    Ok(())
}
fn display_error(e: sideko_rest_api::Error) {
    match &e {
        sideko_rest_api::Error::Io(error) => log::debug!("IO Error: {:?}", error),
        sideko_rest_api::Error::Request(error) => {
            log::debug!("Request Error: {:?}", error)
        }
        sideko_rest_api::Error::Api(api_error)
        | sideko_rest_api::Error::ContentType(api_error) => {
            log::debug!("Response headers: {:?}", & api_error.headers);
            if let Ok(val) = api_error.json::<serde_json::Value>() {
                log::debug!(
                    "Body: {}", serde_json::to_string_pretty(& val).unwrap_or_else(| _ |
                    val.to_string())
                );
            } else if let Ok(text) = std::str::from_utf8(&api_error.content) {
                log::debug!("Body: {text}",);
            } else {
                log::debug!("Unable to process body ({} bytes)", api_error.content.len())
            }
        }
        sideko_rest_api::Error::DeserializeJson(error, _json_str) => {
            log::debug!("JSON Error: {error}")
        }
    }
    log::error!("{e}");
}
#[allow(unused)]
fn save_binary_response(
    res: sideko_rest_api::BinaryResponse,
) -> Result<(), std::io::Error> {
    log::debug!("Binary response headers: {:?}", & res.headers);
    let content_type = res
        .headers
        .get("content-type")
        .map(|val| val.to_str().unwrap_or_default());
    let mut extension = "out".to_string();
    if let Some(ct) = content_type {
        if let Some(Some(suffix)) = mime_guess::get_mime_extensions_str(ct)
            .map(|s| s.first())
        {
            extension = suffix.to_string()
        } else {
            log::warn!("Unsable to determine file extension from content type '{ct}'")
        }
    } else {
        log::warn!(
            "Unable to determine file extension from empty content type header in response"
        )
    }
    let outpath = camino::Utf8PathBuf::from(format!("./output.{extension}"));
    std::fs::write(&outpath, &res.content)?;
    log::info!("Wrote {} bytes to {outpath}", res.content.len());
    Ok(())
}
fn get_dotsideko_path() -> camino::Utf8PathBuf {
    if let Ok(custom_dotsideko) = std::env::var("SIDEKO_CLI_CONFIG") {
        camino::Utf8PathBuf::from_str(&custom_dotsideko)
            .unwrap_or_else(|_| {
                log::debug!("$SIDEKO_CLI_CONFIG set to: '{custom_dotsideko}'");
                log::error!(
                    "$SIDEKO_CLI_CONFIG environment variable must be a valid path if set"
                );
                std::process::exit(1)
            })
    } else {
        let home = std::env::var("HOME")
            .unwrap_or_else(|_| {
                log::error!(
                    "$HOME environment variable must be set for the CLI to function"
                );
                std::process::exit(1)
            });
        let buf = camino::Utf8PathBuf::from_str(&home)
            .unwrap_or_else(|_| {
                log::debug!("$HOME set to: '{home}'");
                log::error!("$HOME environment variable must be a valid path");
                std::process::exit(1)
            });
        buf.join(".sideko-cli")
    }
}
fn load_dotsideko() {
    let path = get_dotsideko_path();
    if path.exists() {
        log::debug!("Loading CLI config from {path}");
        if let Err(e) = dotenv::from_path(path.clone()) {
            log::debug!("Dotenv error: {:?}", e);
            log::error!("Failed loading config from '{path}'");
            std::process::exit(1);
        }
        log::debug!("Loaded config from {path}")
    }
}
fn write_dotsideko(var: &str, val: &str, unset: bool) {
    let sh_safe_val = shlex::try_quote(val)
        .map(String::from)
        .unwrap_or_else(|_| val.to_string());
    let dotenv_entry = format!("{var}={sh_safe_val}");
    let path = get_dotsideko_path();
    let current_dotenv: Vec<String> = if path.exists() {
        let dotenv_string = std::fs::read_to_string(path.clone())
            .unwrap_or_else(|e| {
                log::debug!("FS error: {:?}", e);
                log::error!("Failed loading config from '{path}'");
                std::process::exit(1);
            });
        dotenv_string.split("\n").map(String::from).collect()
    } else {
        vec![]
    };
    let mut new_dotenv: Vec<String> = vec![];
    let mut replaced = false;
    for line in current_dotenv {
        if line.starts_with(&format!("{var}=")) {
            if !unset {
                new_dotenv.push(dotenv_entry.clone());
                replaced = true;
            }
        } else {
            new_dotenv.push(line);
        }
    }
    if !unset && !replaced {
        new_dotenv.push(dotenv_entry)
    }
    std::fs::write(&path, new_dotenv.join("\n"))
        .unwrap_or_else(|e| {
            log::debug!("FS error: {:?}", e);
            log::error!("Failed updating config at '{path}'");
            std::process::exit(1);
        });
    if unset {
        log::debug!("{var} unset in {path}")
    } else {
        log::debug!("{var} updated in {path}")
    }
}
fn init_logger(verbosity: u8) {
    let self_module = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
    let mut builder = env_logger::builder();
    if verbosity == 0 {
        builder
            .filter_module(&self_module, log::LevelFilter::Info)
            .format_target(false)
            .format_timestamp(None)
    } else if verbosity == 1 {
        builder.filter_module(&self_module, log::LevelFilter::Debug).format_target(false)
    } else {
        builder.filter_level(log::LevelFilter::Trace)
    };
    let _ = builder.try_init();
}
fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue))),
        )
        .literal(
            anstyle::Style::new()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}
#[derive(clap::Subcommand)]
enum SidekoConfigSubcommand {
    /// Generate markdown documentation for this CLI
    #[command(name = "docs")]
    Docs {
        /// Sets custom output path
        #[arg(long, default_value = "./CLI.md")]
        output: camino::Utf8PathBuf,
    },
    /// Output shell-autocompletion for this CLI
    /// (output to be piped into relevant rc file for sourcing)
    #[command(name = "completions")]
    Completions { #[arg(long)] shell: clap_complete::Shell },
    /// Configure a custom base url for the CLI to use
    #[command(name = "base-url")]
    BaseUrl {
        /// Base URL to use in future API requests
        #[arg(long)]
        url: Option<String>,
        /// Clear previously set custom base url in favour of default
        #[arg(long)]
        unset: bool,
    },
    /// Add authentication credentials to the CLI
    #[command(subcommand, name = "auth")]
    SidekoAuthSubcommand(SidekoAuthSubcommand),
}
#[derive(clap::Subcommand)]
enum SidekoAuthSubcommand {
    /// Add API key to the CLI for the 'ApiKeyAuth' authentication method
    #[command(name = "api-key-auth")]
    ApiKeyAuth { #[arg(long)] key: String },
    /// Add API key to the CLI for the 'CookieAuth' authentication method
    #[command(name = "cookie-auth")]
    CookieAuth { #[arg(long)] key: String },
}
#[derive(clap::Parser)]
#[command(version, propagate_version = true, name = "sideko-rest-api")]
struct SidekoCli {
    #[command(subcommand)]
    command: SidekoCommand,
    #[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
        global = true,
        help = "Increase logging verbosity"
    )]
    verbose: u8,
}
#[derive(clap::Parser)]
#[command(styles = get_styles())]
#[allow(clippy::enum_variant_names)]
enum SidekoCommand {
    /// command group: authentication/documenention/configurations/etc.
    #[command(subcommand, name = "config")]
    SidekoConfigSubcommand(SidekoConfigSubcommand),
    /// command group (5 commands, 1 sub groups)
    #[command(subcommand, name = "api")]
    ApiSubcommand(ApiSubcommand),
    /// command group (3 commands, 0 sub groups)
    #[command(subcommand, name = "role")]
    RoleSubcommand(RoleSubcommand),
    /// command group (4 commands, 1 sub groups)
    #[command(subcommand, name = "sdk")]
    SdkSubcommand(SdkSubcommand),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api delete-api-link --id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "delete-api-link")]
    DeleteApiLink(sideko_rest_api::DeleteApiLinkRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api delete-api-link-group --id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "delete-api-link-group")]
    DeleteApiLinkGroup(sideko_rest_api::DeleteApiLinkGroupRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api delete-doc-project --doc-name my-project`
    #[command(name = "delete-doc-project")]
    DeleteDocProject(sideko_rest_api::DeleteDocProjectRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api delete-guide --doc-name my-project --doc-version 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --guide-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "delete-guide")]
    DeleteGuide(sideko_rest_api::DeleteGuideRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api delete-guide-href --doc-name my-project --doc-version 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --guide-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --variant next`
    #[command(name = "delete-guide-href")]
    DeleteGuideHref(sideko_rest_api::DeleteGuideHrefRequest),
    /// Delete a media asset in your organization
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api delete-asset --id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "delete-asset")]
    DeleteAsset(sideko_rest_api::DeleteAssetRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api delete-service-account --id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "delete-service-account")]
    DeleteServiceAccount(sideko_rest_api::DeleteServiceAccountRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api list-api-links --doc-version 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "list-api-links")]
    ListApiLinks(sideko_rest_api::ListApiLinksRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-api-link --id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "get-api-link")]
    GetApiLink(sideko_rest_api::GetApiLinkRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api list-api-link-groups --doc-version 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "list-api-link-groups")]
    ListApiLinkGroups(sideko_rest_api::ListApiLinkGroupsRequest),
    /// command
    ///
    /// **Example:** `sideko-rest-api exchange-code-for-key --code string`
    #[command(name = "exchange-code-for-key")]
    ExchangeCodeForKey(sideko_rest_api::ExchangeCodeForKeyRequest),
    /// command
    ///
    /// **Example:** `sideko-rest-api login-callback --code string --state string`
    #[command(name = "login-callback")]
    LoginCallback(sideko_rest_api::LoginCallbackRequest),
    /// command
    ///
    /// **Example:** `sideko-rest-api login-url --cli-output string --cli-port 123`
    #[command(name = "login-url")]
    LoginUrl(sideko_rest_api::LoginUrlRequest),
    /// command
    ///
    /// **Example:** `sideko-rest-api cli-check-updates --cli-version 0.1.0`
    #[command(name = "cli-check-updates")]
    CliCheckUpdates(sideko_rest_api::CliCheckUpdatesRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api list-doc-projects`
    #[command(name = "list-doc-projects")]
    ListDocProjects,
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-doc-project --doc-name my-project`
    #[command(name = "get-doc-project")]
    GetDocProject(sideko_rest_api::GetDocProjectRequest),
    /// Retrieves all deployments for a doc project
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api list-deployments --doc-name my-project --limit 123 --target Preview`
    #[command(name = "list-deployments")]
    ListDeployments(sideko_rest_api::ListDeploymentsRequest),
    /// Retrieves single deployment
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-deployment --doc-name my-project --deployment-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "get-deployment")]
    GetDeployment(sideko_rest_api::GetDeploymentRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-doc-project-theme --doc-name my-project`
    #[command(name = "get-doc-project-theme")]
    GetDocProjectTheme(sideko_rest_api::GetDocProjectThemeRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api list-doc-versions --doc-name my-project`
    #[command(name = "list-doc-versions")]
    ListDocVersions(sideko_rest_api::ListDocVersionsRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-doc-version --doc-name my-project --doc-version 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "get-doc-version")]
    GetDocVersion(sideko_rest_api::GetDocVersionRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api list-guides --doc-name my-project --doc-version 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "list-guides")]
    ListGuides(sideko_rest_api::ListGuidesRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-guide --doc-name my-project --doc-version 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --guide-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "get-guide")]
    GetGuide(sideko_rest_api::GetGuideRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-guide-content --doc-name my-project --doc-version 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --guide-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "get-guide-content")]
    GetGuideContent(sideko_rest_api::GetGuideContentRequest),
    /// Get user organization
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-organization`
    #[command(name = "get-organization")]
    GetOrganization,
    /// Get all media assets for an organization
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api list-assets --name string --page 123`
    #[command(name = "list-assets")]
    ListAssets(sideko_rest_api::ListAssetsRequest),
    /// Get documentation project theme configured at the organization level
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-organization-theme`
    #[command(name = "get-organization-theme")]
    GetOrganizationTheme,
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-current-user`
    #[command(name = "get-current-user")]
    GetCurrentUser,
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-api-key`
    #[command(name = "get-api-key")]
    GetApiKey,
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api list-service-accounts`
    #[command(name = "list-service-accounts")]
    ListServiceAccounts,
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api get-service-account --id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "get-service-account")]
    GetServiceAccount(sideko_rest_api::GetServiceAccountRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api update-api-link --id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --api-version '{"api_id":"my-api","version":"0.1.0"}' --build-request-enabled true --include-mock-server true --nav-label string --policy latest --slug string`
    #[command(name = "update-api-link")]
    UpdateApiLink(sideko_rest_api::UpdateApiLinkRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api update-api-link-group --id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --nav-label string --slug string`
    #[command(name = "update-api-link-group")]
    UpdateApiLinkGroup(sideko_rest_api::UpdateApiLinkGroupRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api update-doc-project --doc-name my-project --logos '{"dark":"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a","favicon":"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a","light":"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"}' --name my-company-docs --settings '{"action_button":{"enabled":true,"label":"string","url":"http://www.example.com"},"metadata":{"description":"string","title":"string"}}'`
    #[command(name = "update-doc-project")]
    UpdateDocProject(sideko_rest_api::UpdateDocProjectRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api update-guide --doc-name my-project --doc-version 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --guide-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --content string --nav-label string --next-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --prev-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --slug string`
    #[command(name = "update-guide")]
    UpdateGuide(sideko_rest_api::UpdateGuideRequest),
    /// Update a media asset in your organization
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api update-asset --id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --name string`
    #[command(name = "update-asset")]
    UpdateAsset(sideko_rest_api::UpdateAssetRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api create-api-link --build-request-enabled true --doc-version-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --group-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --include-mock-server true --nav-label string --policy '{"api_id":"my-api","type":"latest"}' --slug string`
    #[command(name = "create-api-link")]
    CreateApiLink(sideko_rest_api::CreateApiLinkRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api reorder-api-links --doc-version-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --groups '{"id":"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a","order":123}' --links '{"group_id":"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a","id":"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a","order":123}'`
    #[command(name = "reorder-api-links")]
    ReorderApiLinks(sideko_rest_api::ReorderApiLinksRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api create-api-link-group --doc-version-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --nav-label string --slug string`
    #[command(name = "create-api-link-group")]
    CreateApiLinkGroup(sideko_rest_api::CreateApiLinkGroupRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api create-doc-project --name my-company-docs`
    #[command(name = "create-doc-project")]
    CreateDocProject(sideko_rest_api::CreateDocProjectRequest),
    /// Deploys a new generated version of documentation with linked guides & APIs
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api trigger-deployment --doc-name my-project --doc-version-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --target Preview`
    #[command(name = "trigger-deployment")]
    TriggerDeployment(sideko_rest_api::TriggerDeploymentRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api create-guide --doc-name my-project --doc-version 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --content string --is-parent --nav-label string --next-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --parent-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --prev-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --slug string`
    #[command(name = "create-guide")]
    CreateGuide(sideko_rest_api::CreateGuideRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api reorder-guides --doc-name my-project --doc-version 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --data '{"id":"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a","order":123,"parent_id":"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"}'`
    #[command(name = "reorder-guides")]
    ReorderGuides(sideko_rest_api::ReorderGuidesRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api create-organization --name string --subdomain string`
    #[command(name = "create-organization")]
    CreateOrganization(sideko_rest_api::CreateOrganizationRequest),
    /// Add a media asset like logos or other media to an organization
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api upload-assets --file uploads/file.pdf`
    #[command(name = "upload-assets")]
    UploadAssets(sideko_rest_api::UploadAssetsRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api invite-user --email user@example.com --role-definition-id ApiProjectAdmin`
    #[command(name = "invite-user")]
    InviteUser(sideko_rest_api::InviteUserRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api create-service-account --name 'Documentation Publisher Service Account' --object-roles '{"object_id":"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a","object_type":"api_project","role_definition_id":"ApiProjectAdmin"}'`
    #[command(name = "create-service-account")]
    CreateServiceAccount(sideko_rest_api::CreateServiceAccountRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api update-doc-project-theme --doc-name my-project --api-reference-group-variant grouped --dark-active-button-bg-color '#FFFFFF' --dark-active-button-text-color '#FFFFFF' --dark-bg-color '#FFFFFF' --dark-navbar-color '#FFFFFF' --dark-navbar-text-color '#FFFFFF' --light-active-button-bg-color '#FFFFFF' --light-active-button-text-color '#FFFFFF' --light-bg-color '#FFFFFF' --light-navbar-color '#FFFFFF' --light-navbar-text-color '#FFFFFF'`
    #[command(name = "update-doc-project-theme")]
    UpdateDocProjectTheme(sideko_rest_api::UpdateDocProjectThemeRequest),
    /// Update  documentation project theme configured at the organization level
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api update-organization-theme --api-reference-group-variant grouped --dark-active-button-bg-color '#FFFFFF' --dark-active-button-text-color '#FFFFFF' --dark-bg-color '#FFFFFF' --dark-navbar-color '#FFFFFF' --dark-navbar-text-color '#FFFFFF' --light-active-button-bg-color '#FFFFFF' --light-active-button-text-color '#FFFFFF' --light-bg-color '#FFFFFF' --light-navbar-color '#FFFFFF' --light-navbar-text-color '#FFFFFF'`
    #[command(name = "update-organization-theme")]
    UpdateOrganizationTheme(sideko_rest_api::UpdateOrganizationThemeRequest),
}
#[derive(clap::Subcommand)]
#[allow(clippy::enum_variant_names)]
enum ApiSubcommand {
    /// command group (5 commands, 2 sub groups)
    #[command(subcommand, name = "spec")]
    ApiSpecSubcommand(ApiSpecSubcommand),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api delete --api-name my-project`
    #[command(name = "delete")]
    Delete(sideko_rest_api::resources::api::DeleteRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api list`
    #[command(name = "list")]
    List,
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api get --api-name my-project`
    #[command(name = "get")]
    Get(sideko_rest_api::resources::api::GetRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api patch --api-name my-project --name my-new-api-name`
    #[command(name = "patch")]
    Patch(sideko_rest_api::resources::api::PatchRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api create --name my-api-spec`
    #[command(name = "create")]
    Create(sideko_rest_api::resources::api::CreateRequest),
}
#[derive(clap::Subcommand)]
#[allow(clippy::enum_variant_names)]
enum ApiSpecSubcommand {
    /// command group (1 commands, 0 sub groups)
    #[command(subcommand, name = "openapi")]
    ApiSpecOpenapiSubcommand(ApiSpecOpenapiSubcommand),
    /// command group (1 commands, 0 sub groups)
    #[command(subcommand, name = "stats")]
    ApiSpecStatsSubcommand(ApiSpecStatsSubcommand),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api spec delete --api-name my-project --api-version latest`
    #[command(name = "delete")]
    Delete(sideko_rest_api::resources::api::spec::DeleteRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api spec list --api-name my-project`
    #[command(name = "list")]
    List(sideko_rest_api::resources::api::spec::ListRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api spec get --api-name my-project --api-version latest`
    #[command(name = "get")]
    Get(sideko_rest_api::resources::api::spec::GetRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api spec patch --api-name my-project --api-version latest --mock-server-enabled true --notes '<p>This version includes a number of excellent improvements</p>' --openapi openapi.yaml --version string`
    #[command(name = "patch")]
    Patch(sideko_rest_api::resources::api::spec::PatchRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api spec create --api-name my-project --mock-server-enabled true --notes '<p>This version includes a number of excellent improvements</p>' --openapi openapi.yaml --version major`
    #[command(name = "create")]
    Create(sideko_rest_api::resources::api::spec::CreateRequest),
}
#[derive(clap::Subcommand)]
#[allow(clippy::enum_variant_names)]
enum ApiSpecOpenapiSubcommand {
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api spec openapi get-openapi --api-name my-project --api-version latest`
    #[command(name = "get-openapi")]
    GetOpenapi(sideko_rest_api::resources::api::spec::openapi::GetOpenapiRequest),
}
#[derive(clap::Subcommand)]
#[allow(clippy::enum_variant_names)]
enum ApiSpecStatsSubcommand {
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api api spec stats get-stats --api-name my-project --api-version latest`
    #[command(name = "get-stats")]
    GetStats(sideko_rest_api::resources::api::spec::stats::GetStatsRequest),
}
#[derive(clap::Subcommand)]
#[allow(clippy::enum_variant_names)]
enum RoleSubcommand {
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api role delete --id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "delete")]
    Delete(sideko_rest_api::resources::role::DeleteRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api role list --object-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --object-type api_project --user-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "list")]
    List(sideko_rest_api::resources::role::ListRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api role create --object-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --object-type api_project --role-definition-id ApiProjectAdmin --user-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a`
    #[command(name = "create")]
    Create(sideko_rest_api::resources::role::CreateRequest),
}
#[derive(clap::Subcommand)]
#[allow(clippy::enum_variant_names)]
enum SdkSubcommand {
    /// command group (2 commands, 0 sub groups)
    #[command(subcommand, name = "config")]
    SdkConfigSubcommand(SdkConfigSubcommand),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api sdk list --api-name my-project --successful true`
    #[command(name = "list")]
    List(sideko_rest_api::resources::sdk::ListRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api sdk generate --api-version latest --config uploads/file.pdf --language go --sdk-version 0.1.0`
    #[command(name = "generate")]
    Generate(sideko_rest_api::resources::sdk::GenerateRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api sdk update --api-version latest --config uploads/file.pdf --prev-sdk-git uploads/file.pdf --prev-sdk-id 3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a --sdk-version major`
    #[command(name = "update")]
    Update(sideko_rest_api::resources::sdk::UpdateRequest),
    /// command
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api sdk generate-stateless --base-url http://127.0.0.1:8080/api --language go --openapi openapi.yaml --package-name my_sdk`
    #[command(name = "generate-stateless")]
    GenerateStateless(sideko_rest_api::resources::sdk::GenerateStatelessRequest),
}
#[derive(clap::Subcommand)]
#[allow(clippy::enum_variant_names)]
enum SdkConfigSubcommand {
    /// Creates a sdk config with default configurations for the api/api version
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api sdk config init --api-name my-project --api-version latest`
    #[command(name = "init")]
    Init(sideko_rest_api::resources::sdk::config::InitRequest),
    /// Updates provided config with missing default configurations for the api version
    ///
    /// **Required Auth:** CookieAuth OR ApiKeyAuth
    ///
    /// **Example:** `sideko-rest-api sdk config sync --api-version latest --config uploads/file.pdf`
    #[command(name = "sync")]
    Sync(sideko_rest_api::resources::sdk::config::SyncRequest),
}
#[cfg(test)]
mod cli_tests {
    use clap::Parser;
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_delete_204_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "api", "delete", "--api-name", "my-project"]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_spec_delete_204_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "api",
                        "spec",
                        "delete",
                        "--api-name",
                        "my-project",
                        "--api-version",
                        "latest",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_delete_api_link_204_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "delete-api-link",
                        "--id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_delete_api_link_group_204_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "delete-api-link-group",
                        "--id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_delete_doc_project_204_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "delete-doc-project",
                        "--doc-name",
                        "my-project",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_delete_guide_204_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "delete-guide",
                        "--doc-name",
                        "my-project",
                        "--doc-version",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--guide-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_delete_guide_href_204_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "delete-guide-href",
                        "--doc-name",
                        "my-project",
                        "--doc-version",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--guide-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--variant",
                        "next",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_delete_asset_204_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "delete-asset",
                        "--id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_role_delete_204_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "role",
                        "delete",
                        "--id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_delete_service_account_204_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "delete-service-account",
                        "--id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_health_check_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(&["sideko-rest-api", "health-check"].join(" ")),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_ping_check_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(&["sideko-rest-api", "ping-check"].join(" ")),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_list_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(&["sideko-rest-api", "api", "list"].join(" ")),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_get_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "api", "get", "--api-name", "my-project"]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_spec_list_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "api",
                        "spec",
                        "list",
                        "--api-name",
                        "my-project",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_spec_get_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "api",
                        "spec",
                        "get",
                        "--api-name",
                        "my-project",
                        "--api-version",
                        "latest",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_spec_openapi_get_openapi_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "api",
                        "spec",
                        "openapi",
                        "get-openapi",
                        "--api-name",
                        "my-project",
                        "--api-version",
                        "latest",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_spec_stats_get_stats_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "api",
                        "spec",
                        "stats",
                        "get-stats",
                        "--api-name",
                        "my-project",
                        "--api-version",
                        "latest",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_list_api_links_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "list-api-links",
                        "--doc-version",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_api_link_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "get-api-link",
                        "--id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_list_api_link_groups_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "list-api-link-groups",
                        "--doc-version",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_exchange_code_for_key_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "exchange-code-for-key", "--code", "string"]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_login_callback_303_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "login-callback",
                        "--code",
                        "string",
                        "--state",
                        "string",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_login_url_303_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "login-url",
                        "--cli-output",
                        "string",
                        "--cli-port",
                        "123",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_cli_check_updates_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "cli-check-updates", "--cli-version", "0.1.0"]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_list_doc_projects_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(&["sideko-rest-api", "list-doc-projects"].join(" ")),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_doc_project_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "get-doc-project", "--doc-name", "my-project"]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_list_deployments_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "list-deployments",
                        "--doc-name",
                        "my-project",
                        "--limit",
                        "123",
                        "--target",
                        "Preview",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_deployment_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "get-deployment",
                        "--doc-name",
                        "my-project",
                        "--deployment-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_check_preview_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "check-preview", "--doc-name", "my-project"]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_doc_project_theme_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "get-doc-project-theme",
                        "--doc-name",
                        "my-project",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_list_doc_versions_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "list-doc-versions", "--doc-name", "my-project"]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_doc_version_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "get-doc-version",
                        "--doc-name",
                        "my-project",
                        "--doc-version",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_list_guides_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "list-guides",
                        "--doc-name",
                        "my-project",
                        "--doc-version",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_guide_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "get-guide",
                        "--doc-name",
                        "my-project",
                        "--doc-version",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--guide-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_guide_content_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "get-guide-content",
                        "--doc-name",
                        "my-project",
                        "--doc-version",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--guide-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_organization_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(&["sideko-rest-api", "get-organization"].join(" ")),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_list_assets_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "list-assets",
                        "--name",
                        "string",
                        "--page",
                        "123",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_organization_theme_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "get-organization-theme"].join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_role_list_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "role",
                        "list",
                        "--object-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--object-type",
                        "api_project",
                        "--user-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_sdk_list_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "sdk",
                        "list",
                        "--api-name",
                        "my-project",
                        "--successful",
                        "true",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_current_user_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(&["sideko-rest-api", "get-current-user"].join(" ")),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_api_key_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(&["sideko-rest-api", "get-api-key"].join(" ")),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_list_service_accounts_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "list-service-accounts"].join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_get_service_account_200_generated_success() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "get-service-account",
                        "--id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_patch_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "api",
                        "patch",
                        "--api-name",
                        "my-project",
                        "--name",
                        "my-new-api-name",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_spec_patch_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "api",
                        "spec",
                        "patch",
                        "--api-name",
                        "my-project",
                        "--api-version",
                        "latest",
                        "--mock-server-enabled",
                        "true",
                        "--notes",
                        "'<p>This version includes a number of excellent improvements</p>'",
                        "--openapi",
                        "openapi.yaml",
                        "--version",
                        "string",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_update_api_link_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "update-api-link",
                        "--id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--api-version",
                        "'{\"api_id\":\"my-api\",\"version\":\"0.1.0\"}'",
                        "--build-request-enabled",
                        "true",
                        "--include-mock-server",
                        "true",
                        "--nav-label",
                        "string",
                        "--policy",
                        "latest",
                        "--slug",
                        "string",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_update_api_link_group_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "update-api-link-group",
                        "--id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--nav-label",
                        "string",
                        "--slug",
                        "string",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_update_doc_project_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "update-doc-project",
                        "--doc-name",
                        "my-project",
                        "--logos",
                        "'{\"dark\":\"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a\",\"favicon\":\"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a\",\"light\":\"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a\"}'",
                        "--name",
                        "my-company-docs",
                        "--settings",
                        "'{\"action_button\":{\"enabled\":true,\"label\":\"string\",\"url\":\"http://www.example.com\"},\"metadata\":{\"description\":\"string\",\"title\":\"string\"}}'",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_update_guide_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "update-guide",
                        "--doc-name",
                        "my-project",
                        "--doc-version",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--guide-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--content",
                        "string",
                        "--nav-label",
                        "string",
                        "--next-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--prev-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--slug",
                        "string",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_update_asset_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "update-asset",
                        "--id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--name",
                        "string",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_create_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "api", "create", "--name", "my-api-spec"]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_api_spec_create_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "api",
                        "spec",
                        "create",
                        "--api-name",
                        "my-project",
                        "--mock-server-enabled",
                        "true",
                        "--notes",
                        "'<p>This version includes a number of excellent improvements</p>'",
                        "--openapi",
                        "openapi.yaml",
                        "--version",
                        "major",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_create_api_link_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "create-api-link",
                        "--build-request-enabled",
                        "true",
                        "--doc-version-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--group-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--include-mock-server",
                        "true",
                        "--nav-label",
                        "string",
                        "--policy",
                        "'{\"api_id\":\"my-api\",\"type\":\"latest\"}'",
                        "--slug",
                        "string",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_reorder_api_links_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "reorder-api-links",
                        "--doc-version-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--groups",
                        "'{\"id\":\"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a\",\"order\":123}'",
                        "--links",
                        "'{\"group_id\":\"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a\",\"id\":\"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a\",\"order\":123}'",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_create_api_link_group_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "create-api-link-group",
                        "--doc-version-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--nav-label",
                        "string",
                        "--slug",
                        "string",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_create_doc_project_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "create-doc-project",
                        "--name",
                        "my-company-docs",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_trigger_deployment_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "trigger-deployment",
                        "--doc-name",
                        "my-project",
                        "--doc-version-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--target",
                        "Preview",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_create_guide_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "create-guide",
                        "--doc-name",
                        "my-project",
                        "--doc-version",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--content",
                        "string",
                        "--is-parent",
                        "--nav-label",
                        "string",
                        "--next-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--parent-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--prev-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--slug",
                        "string",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_reorder_guides_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "reorder-guides",
                        "--doc-name",
                        "my-project",
                        "--doc-version",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--data",
                        "'{\"id\":\"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a\",\"order\":123,\"parent_id\":\"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a\"}'",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_create_organization_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "create-organization",
                        "--name",
                        "string",
                        "--subdomain",
                        "string",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_upload_assets_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "upload-assets", "--file", "uploads/file.pdf"]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_role_create_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "role",
                        "create",
                        "--object-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--object-type",
                        "api_project",
                        "--role-definition-id",
                        "ApiProjectAdmin",
                        "--user-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_sdk_generate_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "sdk",
                        "generate",
                        "--api-version",
                        "latest",
                        "--config",
                        "uploads/file.pdf",
                        "--language",
                        "go",
                        "--sdk-version",
                        "0.1.0",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_sdk_config_init_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "sdk",
                        "config",
                        "init",
                        "--api-name",
                        "my-project",
                        "--api-version",
                        "latest",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_sdk_config_sync_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "sdk",
                        "config",
                        "sync",
                        "--api-version",
                        "latest",
                        "--config",
                        "uploads/file.pdf",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_sdk_update_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "sdk",
                        "update",
                        "--api-version",
                        "latest",
                        "--config",
                        "uploads/file.pdf",
                        "--prev-sdk-git",
                        "uploads/file.pdf",
                        "--prev-sdk-id",
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a",
                        "--sdk-version",
                        "major",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_sdk_generate_stateless_201_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "sdk",
                        "generate-stateless",
                        "--base-url",
                        "http://127.0.0.1:8080/api",
                        "--language",
                        "go",
                        "--openapi",
                        "openapi.yaml",
                        "--package-name",
                        "my_sdk",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_invite_user_202_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "invite-user",
                        "--email",
                        "user@example.com",
                        "--role-definition-id",
                        "ApiProjectAdmin",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_create_service_account_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "create-service-account",
                        "--name",
                        "'Documentation Publisher Service Account'",
                        "--object-roles",
                        "'{\"object_id\":\"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a\",\"object_type\":\"api_project\",\"role_definition_id\":\"ApiProjectAdmin\"}'",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_vercel_webhook_202_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &["sideko-rest-api", "vercel-webhook", "--data", "'{}'"].join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_update_doc_project_theme_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "update-doc-project-theme",
                        "--doc-name",
                        "my-project",
                        "--api-reference-group-variant",
                        "grouped",
                        "--dark-active-button-bg-color",
                        "'#FFFFFF'",
                        "--dark-active-button-text-color",
                        "'#FFFFFF'",
                        "--dark-bg-color",
                        "'#FFFFFF'",
                        "--dark-navbar-color",
                        "'#FFFFFF'",
                        "--dark-navbar-text-color",
                        "'#FFFFFF'",
                        "--light-active-button-bg-color",
                        "'#FFFFFF'",
                        "--light-active-button-text-color",
                        "'#FFFFFF'",
                        "--light-bg-color",
                        "'#FFFFFF'",
                        "--light-navbar-color",
                        "'#FFFFFF'",
                        "--light-navbar-text-color",
                        "'#FFFFFF'",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
    #[serial_test::serial]
    #[tokio::test]
    async fn test_cli_update_organization_theme_200_success_default() {
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "api-key-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "config",
                        "auth",
                        "cookie-auth",
                        "--key",
                        "API_KEY",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing auth cli input");
        super::handle_cli(cli).await.expect("failed running auth command");
        let cli = super::SidekoCli::try_parse_from(
                shlex::Shlex::new(
                    &[
                        "sideko-rest-api",
                        "update-organization-theme",
                        "--api-reference-group-variant",
                        "grouped",
                        "--dark-active-button-bg-color",
                        "'#FFFFFF'",
                        "--dark-active-button-text-color",
                        "'#FFFFFF'",
                        "--dark-bg-color",
                        "'#FFFFFF'",
                        "--dark-navbar-color",
                        "'#FFFFFF'",
                        "--dark-navbar-text-color",
                        "'#FFFFFF'",
                        "--light-active-button-bg-color",
                        "'#FFFFFF'",
                        "--light-active-button-text-color",
                        "'#FFFFFF'",
                        "--light-bg-color",
                        "'#FFFFFF'",
                        "--light-navbar-color",
                        "'#FFFFFF'",
                        "--light-navbar-text-color",
                        "'#FFFFFF'",
                    ]
                        .join(" "),
                ),
            )
            .expect("failed parsing cli input");
        let result = super::handle_cli(cli).await;
        println!("{:?}", & result);
        assert!(result.is_ok())
    }
}
