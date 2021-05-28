use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct TagField {
    tag: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ServerBasicInfo {
    core_number: String,
    created: u64,
    host: u64,
    hostname: String,
    license: Decimal,
    memory_amount: String,
    plan: String,
    plan_ipv4_bytes: String,
    plan_ipv6_bytes: String,
    simple_backup: String,
    state: String,
    tags: TagField,
    title: String,
    uuid: String,
    zone: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetServerResponseServersField {
    server: Vec<ServerBasicInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetServerResponse {
    servers: GetServerResponseServersField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IpAddressInfo {
    pub access: String,
    pub address: String,
    family: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IpAddressesField {
    pub ip_address: Vec<IpAddressInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateServerResponseField {
    // boot_order: String,
    // core_number: String,
    // created: u64,
    // firewall: String,
    // hostname: String,
    pub ip_addresses: IpAddressesField,
    // license: u64,
    // memory_amount: String,
    // metadata: String,
    // networking: serde_json::Value,
    // nic_model: String,
    // password: String,
    // plan: String,
    // plan_ipv4_bytes: String,
    // plan_ipv6_bytes: String,
    // progress: String,
    // remote_access_enabled: String,
    // remote_access_password: String,
    // remote_access_type: String,
    // simple_backup: String,
    // state: String,
    // storage_devices: serde_json::Value,
    // tags: TagField,
    // timezone: String,
    // title: String,
    // username: String,
    // uuid: String,
    // video_model: String,
    // zone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateServerResponse {
    pub server: CreateServerResponseField,
}

pub async fn get_servers(ctx: &Context) -> Result<Vec<ServerBasicInfo>, reqwest::Error> {
    let url = format!("{}/1.3/server", ctx.uc_baseurl);
    let client = reqwest::Client::new();
    let request: Result<reqwest::Response, reqwest::Error> = client
        .get(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .send()
        .await;
    let response: Result<GetServerResponse, _> = match request {
        Ok(response) => response.json::<GetServerResponse>().await,
        Err(error) => {
            panic!("{}", error);
        }
    };

    return match response {
        Ok(payload) => Ok(payload.servers.server),
        Err(err) => panic!("{}", err),
    };
}

pub async fn delete(ctx: &Context, uuid: String) -> Result<(), reqwest::Error> {
    let url = format!("{}/1.3/server/{}", ctx.uc_baseurl, uuid);
    let client = reqwest::Client::new();
    let request: Result<reqwest::Response, reqwest::Error> = client
        .delete(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .send()
        .await;
    let result = match request {
        Ok(response) => match response.status() {
            reqwest::StatusCode::NO_CONTENT => Ok(()),
            _ => panic!(),
        },
        Err(err) => Err(err),
    };
    return result;
}

pub async fn delete_along_storage(ctx: &Context, uuid: String) -> Result<(), reqwest::Error> {
    let url = format!("{}/1.3/server/{}/?storages=1", ctx.uc_baseurl, uuid);
    let client = reqwest::Client::new();
    let request: Result<reqwest::Response, reqwest::Error> = client
        .delete(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .send()
        .await;
    let result = match request {
        Ok(response) => match response.status() {
            reqwest::StatusCode::NO_CONTENT => Ok(()),
            _ => panic!(),
        },
        Err(err) => Err(err),
    };
    return result;
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StopServerField {
    stop_type: String,
    timeout: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StopServerPayload {
    stop_server: StopServerField,
}

pub async fn stop(ctx: &Context, uuid: String) -> Result<(), reqwest::Error> {
    let url = format!("{}/1.3/server/{}/stop", ctx.uc_baseurl, uuid);
    let client = reqwest::Client::new();
    let payload_body = StopServerPayload {
        stop_server: StopServerField {
            stop_type: String::from("soft"),
            timeout: String::from("30"),
        },
    };
    let request: Result<reqwest::Response, reqwest::Error> = client
        .post(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .json(&payload_body)
        .send()
        .await;
    let result = match request {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => panic!(),
        },
        Err(err) => Err(err),
    };
    return result;
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StartServerStorageDevice {
    action: String,
    storage: String,
    title: String,
    size: usize,
    tier: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StartServerStorageDevicesField {
    storage_device: Vec<StartServerStorageDevice>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct IpAddressSpecs {
    family: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct IpAddressThing {
    ip_address: Vec<IpAddressSpecs>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct IpAddrTypePair {
    ip_addresses: IpAddressThing,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    ip_address_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StartServerInterfacesField {
    interface: Vec<IpAddrTypePair>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StartServerNetworkingField {
    interfaces: StartServerInterfacesField,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StartServerSshKeysField {
    ssh_key: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StartServerLoginUserField {
    username: String,
    ssh_keys: StartServerSshKeysField,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StartServerField {
    zone: String,
    title: String,
    hostname: String,
    plan: String,
    storage_devices: StartServerStorageDevicesField,
    networking: StartServerNetworkingField,
    login_user: StartServerLoginUserField,
    user_data: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StartServerPayload {
    server: StartServerField,
}

pub async fn start(ctx: &Context, uuid: String) -> Result<(), reqwest::Error> {
    let url = format!("{}/1.3/server/{}/start", ctx.uc_baseurl, uuid);
    let client = reqwest::Client::new();
    let payload_body = StopServerPayload {
        stop_server: StopServerField {
            stop_type: String::from("soft"),
            timeout: String::from("30"),
        },
    };
    let request: Result<reqwest::Response, reqwest::Error> = client
        .post(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .json(&payload_body)
        .send()
        .await;
    let result = match request {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => panic!(),
        },
        Err(err) => Err(err),
    };
    return result;
}

pub async fn create_from_template(
    ctx: &Context,
    template_uuid: String,
    hostname: String,
    title: String,
    storage_title: String,
    storage_size: usize,
    plan_name: String,
    zone: String,
    user_data: String,
    username: String,
    ssh_keys: Vec<String>,
) -> Result<CreateServerResponse, reqwest::Error> {
    let url = format!("{}/1.3/server/", ctx.uc_baseurl);
    let client = reqwest::Client::new();
    let payload_body = StartServerPayload {
        server: StartServerField {
            zone,
            title,
            hostname,
            plan: plan_name,
            storage_devices: StartServerStorageDevicesField {
                storage_device: vec![StartServerStorageDevice {
                    action: String::from("clone"),
                    storage: template_uuid,
                    title: storage_title,
                    size: storage_size,
                    tier: String::from("maxiops"),
                }],
            },
            networking: StartServerNetworkingField {
                interfaces: StartServerInterfacesField {
                    interface: vec![
                        IpAddrTypePair {
                            ip_addresses: IpAddressThing {
                                ip_address: vec![IpAddressSpecs {
                                    family: String::from("IPv4"),
                                }],
                            },
                            ip_address_type: String::from("public"),
                        },
                        //   IpAddrTypePair {
                        //     ip_addresses: IpAddressThing { ip_address: vec![ IpAddressSpecs { family: String::from("IPv4") }] },
                        //     ip_address_type: String::from("utility")
                        //   },
                        //   IpAddrTypePair {
                        //     ip_addresses: IpAddressThing { ip_address: vec![ IpAddressSpecs { family: String::from("IPv6") }] },
                        //     ip_address_type: String::from("public")
                        //   }
                    ],
                },
            },
            login_user: StartServerLoginUserField {
                username,
                ssh_keys: StartServerSshKeysField { ssh_key: ssh_keys },
            },
            user_data,
        },
    };

    let json_response = client
        .post(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .json(&payload_body)
        .send()
        .await?
        .json()
        .await?;

    return Ok(json_response);
}
