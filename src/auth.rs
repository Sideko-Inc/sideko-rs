///Generated by Sideko (sideko.dev)
#[derive(Clone, Debug)]
pub enum AuthProvider {
    #[allow(dead_code)]
    Basic(String, String),
    #[allow(dead_code)]
    KeyQuery(String, String),
    #[allow(dead_code)]
    KeyHeader(String, String),
    #[allow(dead_code)]
    KeyCookie(String, String),
    #[allow(dead_code)]
    Bearer(String),
}
impl AuthProvider {
    pub fn add_auth_blocking(
        &self,
        mut builder: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        match self {
            AuthProvider::Basic(username, password) => {
                let unhashed = format!("{username}:{password}");
                let hashed = base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    unhashed,
                );
                let basic_auth = format!("Basic {hashed}");
                builder = builder.header(reqwest::header::AUTHORIZATION, basic_auth);
            }
            AuthProvider::KeyQuery(query_name, key_val) => {
                builder = builder.query(&[(query_name, key_val)]);
            }
            AuthProvider::KeyHeader(header_name, key_val) => {
                builder = builder.header(header_name, key_val);
            }
            AuthProvider::KeyCookie(cookie_name, key_val) => {
                let cookie_val = format!("{cookie_name}={key_val}");
                builder = builder.header(reqwest::header::COOKIE, cookie_val);
            }
            AuthProvider::Bearer(token_val) => {
                builder = builder
                    .header(
                        reqwest::header::AUTHORIZATION,
                        format!("Bearer {token_val}"),
                    );
            }
        };
        builder
    }
    pub fn add_auth(
        &self,
        mut builder: reqwest::RequestBuilder,
    ) -> reqwest::RequestBuilder {
        match self {
            AuthProvider::Basic(username, password) => {
                let unhashed = format!("{username}:{password}");
                let hashed = base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    unhashed,
                );
                let basic_auth = format!("Basic {hashed}");
                builder = builder.header(reqwest::header::AUTHORIZATION, basic_auth);
            }
            AuthProvider::KeyQuery(query_name, key_val) => {
                builder = builder.query(&[(query_name, key_val)]);
            }
            AuthProvider::KeyHeader(header_name, key_val) => {
                builder = builder.header(header_name, key_val);
            }
            AuthProvider::KeyCookie(cookie_name, key_val) => {
                let cookie_val = format!("{cookie_name}={key_val}");
                builder = builder.header(reqwest::header::COOKIE, cookie_val);
            }
            AuthProvider::Bearer(token_val) => {
                builder = builder
                    .header(
                        reqwest::header::AUTHORIZATION,
                        format!("Bearer {token_val}"),
                    );
            }
        };
        builder
    }
}