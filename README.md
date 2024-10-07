
# Sideko REST API Rust SDK


## Overview
The Sideko API unlocks features including generating SDKs, setting up API Specifications with mock servers, creating documentation projects with generated API references and custom pages, managing roles and permissions, and more.


### Example Client Initialization

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
```

### SDK Usage 
 See [SDK Examples](SDK_EXAMPLES.md) for example usage of all SDK functionality