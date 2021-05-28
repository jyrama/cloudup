use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Storage {
    access: String,
    license: Decimal,
    size: usize,
    state: String,
    pub title: String,
    pub uuid: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    storage_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetTemplatesResponseStoragesField {
    storage: Vec<Storage>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetTemplatesResponse {
    storages: GetTemplatesResponseStoragesField,
}

/// GET /1.3/storage/template
pub async fn get_templates(ctx: &Context) -> Result<Vec<Storage>, reqwest::Error> {
    let url = format!("{}/1.3/storage/template", ctx.uc_baseurl);
    let client = reqwest::Client::new();
    let request: Result<reqwest::Response, reqwest::Error> = client
        .get(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .send()
        .await;
    let response: Result<GetTemplatesResponse, _> = match request {
        Ok(response) => response.json::<GetTemplatesResponse>().await,
        Err(error) => return Err(error),
    };
    return match response {
        Ok(payload) => Ok(payload.storages.storage),
        Err(err) => Err(err),
    };
}

pub async fn delete(ctx: &Context, uuid: String) -> Result<(), reqwest::Error> {
    let url = format!("{}/1.3/storage/{}", ctx.uc_baseurl, uuid);
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
