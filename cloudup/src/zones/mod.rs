use crate::Context;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Zone {
    description: String,
    id: String,
    public: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetZonesResponseZoneField {
    zone: Vec<Zone>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetZonesResponse {
    zones: GetZonesResponseZoneField,
}

pub async fn fetch_zones(ctx: &Context) -> Result<GetZonesResponse, reqwest::Error> {
    let url = format!("{}/1.3/zone", ctx.uc_baseurl);
    let client = reqwest::Client::new();
    let request: Result<reqwest::Response, reqwest::Error> = client
        .get(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .send()
        .await;
    let payload = match request {
        Ok(response) => response.json::<GetZonesResponse>(),
        Err(error) => {
            panic!("{}", error);
        }
    };
    payload.await
}
