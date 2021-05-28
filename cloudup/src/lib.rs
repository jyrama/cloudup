pub mod accounts;
pub mod plan;
pub mod pricing;
pub mod server;
pub mod storage;
pub mod zones;

struct Credentials {
    user: String,
    pass: Option<String>,
}

pub struct Context {
    uc_baseurl: &'static str,
    credentials: Credentials,
}

impl Context {
    pub fn new(endpoint: &'static str, user: String, password: String) -> Context {
        return Context {
            uc_baseurl: endpoint,
            credentials: Credentials {
                user,
                pass: Some(password),
            },
        };
    }
}
