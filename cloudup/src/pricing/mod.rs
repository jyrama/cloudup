use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct PriceAmountPair {
    amount: usize,
    price: Decimal,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PricingZone {
    pub name: String,
    firewall: PriceAmountPair,
    io_request_backup: PriceAmountPair,
    io_request_hdd: PriceAmountPair,
    io_request_maxiops: PriceAmountPair,
    ipv4_address: PriceAmountPair,
    ipv6_address: PriceAmountPair,
    network_private_vlan: PriceAmountPair,
    public_ipv4_bandwidth_in: PriceAmountPair,
    public_ipv4_bandwidth_out: PriceAmountPair,
    public_ipv6_bandwidth_in: PriceAmountPair,
    public_ipv6_bandwidth_out: PriceAmountPair,
    server_core: PriceAmountPair,
    server_memory: PriceAmountPair,
    storage_backup: PriceAmountPair,
    storage_hdd: PriceAmountPair,
    storage_maxiops: PriceAmountPair,
    storage_template: PriceAmountPair,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct PricesField {
    zone: Vec<PricingZone>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct GetPricingResponse {
    prices: PricesField,
}

pub async fn get_pricing(ctx: &Context) -> Result<Vec<PricingZone>, reqwest::Error> {
    let url = format!("{}/1.3/price", ctx.uc_baseurl);
    let client = reqwest::Client::new();
    let request: Result<reqwest::Response, reqwest::Error> = client
        .get(&url)
        .basic_auth(ctx.credentials.user.clone(), ctx.credentials.pass.clone())
        .send()
        .await;
    let response: Result<GetPricingResponse, _> = match request {
        Ok(response) => response.json::<GetPricingResponse>().await,
        Err(error) => {
            panic!("{}", error);
        }
    };

    return match response {
        Ok(payload) => Ok(payload.prices.zone),
        Err(err) => panic!("{}", err),
    };
}
