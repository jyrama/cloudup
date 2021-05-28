use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    core_number: usize,
    memory_amount: usize,
    pub name: String,
    public_traffic_out: usize,
    pub storage_size: usize,
    storage_tier: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetPlansResponsePlansField {
    plan: Vec<Plan>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetPlansResponse {
    plans: GetPlansResponsePlansField,
}

pub async fn get_plans(ctx: &Context) -> Result<Vec<Plan>, reqwest::Error> {
    let url = format!("{}/1.3/plan", ctx.uc_baseurl);
    let client = reqwest::Client::new();
    let request: Result<reqwest::Response, reqwest::Error> = client
        .get(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .send()
        .await;
    let response: Result<GetPlansResponse, _> = match request {
        Ok(response) => response.json::<GetPlansResponse>().await,
        Err(error) => {
            panic!("{}", error);
        }
    };

    return match response {
        Ok(payload) => Ok(payload.plans.plan),
        Err(err) => panic!("{}", err),
    };
}
