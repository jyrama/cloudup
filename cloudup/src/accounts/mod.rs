use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ResourceLimits {
    cores: usize,
    // detached_floating_ips: usize,
    memory: usize,
    networks: usize,
    public_ipv4: usize,
    public_ipv6: usize,
    storage_hdd: usize,
    storage_ssd: usize,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Account {
    credits: Decimal,
    username: String,
    resource_limits: ResourceLimits,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetAccountResponse {
    // GET /1.3/account
    account: Account,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct AccountRolesField {
    role: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AccountMinimalInfo {
    roles: AccountRolesField,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    account_type: String,
    username: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetAccountListResponseAccountsField {
    account: Vec<AccountMinimalInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetAccountListResponse {
    // GET /1.3/account/list
    accounts: GetAccountListResponseAccountsField,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetAccountDetailsResponseCampaignsField {
    campaign: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetAccountDetailsResponseNetworkAccessField {
    network: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct ServerAccessPair {
    storage: String,
    uuid: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetAccountDetailsResponseServerAccessField {
    server: Vec<ServerAccessPair>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetAccountDetailsResponseStorageAccessField {
    storage: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetAccountDetailsResponseTagAccessField {
    tag: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetAccountDetailsResponseIpFiltersField {
    ip_filter: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AccountDetails {
    main_account: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    account_type: String,
    username: String,
    first_name: String,
    last_name: String,
    company: String,
    address: String,
    postal_code: String,
    city: String,
    state: String,
    country: String,
    currency: String,
    language: String,
    phone: String,
    email: String,
    vat_number: String,
    timezone: String,
    campaigns: GetAccountDetailsResponseCampaignsField,
    roles: AccountRolesField,
    allow_api: String,
    allow_gui: String,
    enable_3rd_party_services: String,
    network_access: GetAccountDetailsResponseNetworkAccessField,
    server_access: GetAccountDetailsResponseServerAccessField,
    storage_access: GetAccountDetailsResponseStorageAccessField,
    tag_access: GetAccountDetailsResponseTagAccessField,
    ip_filters: GetAccountDetailsResponseIpFiltersField,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetAccountDetailsResponse {
    account: AccountDetails,
}

/// GET /1.3/account -> Returns information on the user's account and resource limits.
pub async fn get_account_info(ctx: &Context) -> Result<Account, reqwest::Error> {
    let url = format!("{}/1.3/account", ctx.uc_baseurl);
    let client = reqwest::Client::new();
    let request: Result<reqwest::Response, reqwest::Error> = client
        .get(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .send()
        .await;
    let response: Result<GetAccountResponse, _> = match request {
        Ok(response) => response.json::<GetAccountResponse>().await,
        Err(error) => Err(error),
    };
    return match response {
        Ok(payload) => Ok(payload.account),
        Err(err) => Err(err),
    };
}

/// GET /1.3/account/list
pub async fn get_account_list(ctx: &Context) -> Result<Vec<AccountMinimalInfo>, reqwest::Error> {
    let url = format!("{}/1.3/account/list", ctx.uc_baseurl);
    let client = reqwest::Client::new();
    let request: Result<reqwest::Response, reqwest::Error> = client
        .get(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .send()
        .await;
    let response: Result<GetAccountListResponse, _> = match request {
        Ok(response) => response.json::<GetAccountListResponse>().await,
        Err(error) => {
            panic!("{}", error);
        }
    };
    return match response {
        Ok(payload) => Ok(payload.accounts.account),
        Err(err) => Err(err),
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    use rust_decimal_macros::*;

    #[test]
    fn get_account() {
        let account = GetAccountResponse {
            account: Account {
                credits: dec!(9972.2324),
                username: String::from("username"),
                resource_limits: ResourceLimits {
                    cores: 200,
                    // detached_floating_ips: 10,
                    memory: 1048576,
                    networks: 100,
                    public_ipv4: 100,
                    public_ipv6: 100,
                    storage_hdd: 10240,
                    storage_ssd: 10240,
                },
            },
        };

        println!("{}", serde_json::to_string(&account).unwrap());
    }

    #[test]
    fn get_account_list() {
        let account_list = GetAccountListResponse {
            accounts: GetAccountListResponseAccountsField {
                account: vec![
                    AccountMinimalInfo {
                        roles: AccountRolesField {
                            role: vec![String::from("technical")],
                        },
                        account_type: String::from("mymain"),
                        username: String::from("test"),
                    },
                    AccountMinimalInfo {
                        roles: AccountRolesField {
                            role: vec![String::from("technical")],
                        },
                        account_type: String::from("sub"),
                        username: String::from("my_sub_account"),
                    },
                    AccountMinimalInfo {
                        roles: AccountRolesField {
                            role: vec![String::from("billing")],
                        },
                        account_type: String::from("sub"),
                        username: String::from("my_billing_account"),
                    },
                ],
            },
        };

        println!("{}", serde_json::to_string(&account_list).unwrap());
    }

    #[test]
    fn get_account_details() {
        let account_details = GetAccountDetailsResponse {
            account: AccountDetails {
                main_account: String::from("mymain"),
                account_type: String::from("sub"),
                username: String::from("my_sub_account"),
                first_name: String::from("first"),
                last_name: String::from("last"),
                company: String::from("UpCloud Ltd"),
                address: String::from("my address"),
                postal_code: String::from("00130"),
                city: String::from("Helsinki"),
                state: String::from(""),
                country: String::from("FIN"),
                currency: String::from("USD"),
                language: String::from("fi"),
                phone: String::from("+358.31245434"),
                email: String::from("test@myhost.mydomain"),
                vat_number: String::from("FI24315605"),
                timezone: String::from("UTC"),
                campaigns: GetAccountDetailsResponseCampaignsField { campaign: vec![] },
                roles: AccountRolesField { role: vec![] },
                allow_api: String::from("yes"),
                allow_gui: String::from("no"),
                enable_3rd_party_services: String::from("yes"),
                network_access: GetAccountDetailsResponseNetworkAccessField { network: vec![] },
                server_access: GetAccountDetailsResponseServerAccessField {
                    server: vec![ServerAccessPair {
                        storage: String::from("no"),
                        uuid: String::from("*"),
                    }],
                },
                storage_access: GetAccountDetailsResponseStorageAccessField {
                    storage: vec![String::from("*")],
                },
                tag_access: GetAccountDetailsResponseTagAccessField { tag: vec![] },
                ip_filters: GetAccountDetailsResponseIpFiltersField { ip_filter: vec![] },
            },
        };

        println!("{}", serde_json::to_string(&account_details).unwrap());
    }
}
