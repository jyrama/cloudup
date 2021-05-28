use std::env;

#[tokio::main]
async fn main() {
    let uc_user = match env::var("UC_USER") {
        Ok(user) => user,
        Err(err) => panic!("Couldn't read UC_USER ({})", err),
    };

    let uc_pass = match env::var("UC_PASS") {
        Ok(pass) => pass,
        Err(err) => panic!("Couldn't read UC_PASS ({})", err),
    };

    let uc_zone = match env::var("UC_ZONE") {
        Ok(uc_zone) => uc_zone,
        Err(err) => panic!("Couldn't read UC_ZONE ({})", err),
    };

    let username = match env::var("USERNAME") {
        Ok(username) => username,
        Err(err) => panic!("Couldn't read USERNAME ({})", err),
    };

    let ssh_pub = match env::var("SSH_PUB") {
        Ok(ssh_pub) => ssh_pub,
        Err(err) => panic!("Couldn't read SSH_PUB ({})", err),
    };

    let server_name = match env::var("SERVER_NAME") {
        Ok(server_name) => server_name,
        Err(err) => panic!("Couldn't read SERVER_NAME ({})", err),
    };

    let ctx = cloudup::Context::new("https://api.upcloud.com", uc_user, uc_pass);

    let plans: Vec<cloudup::plan::Plan> = cloudup::plan::get_plans(&ctx)
        .await
        .expect("Fetching plans failed");

    let desired_plan: &cloudup::plan::Plan = plans
        .iter()
        .find(|x| x.name == "2xCPU-4GB")
        .expect("Desired plan was not found!");

    let templates: Vec<cloudup::storage::Storage> = cloudup::storage::get_templates(&ctx)
        .await
        .expect("Fetching templates failed");

    let centos: &cloudup::storage::Storage = templates
        .iter()
        .find(|x| x.title == "CentOS 8")
        .expect("CentsOS 8 was not found!");

    let servers: Vec<cloudup::server::ServerBasicInfo> = cloudup::server::get_servers(&ctx)
        .await
        .expect("Fetching server list failed");
    println!("{:#?}", servers);


    let api_resp: Result<cloudup::server::CreateServerResponse, reqwest::Error> =
        cloudup::server::create_from_template(
            &ctx,
            centos.uuid.clone(),
            server_name.clone(),
            server_name,
            centos.title.clone(),
            desired_plan.storage_size,
            desired_plan.name.clone(),
            uc_zone,
            String::from(
                "dnf install tmux tar java-1.8.0-openjdk -y && firewall-cmd --add-port 25565/tcp",
            ),
            username,
            vec![ssh_pub]
        )
        .await;

    match api_resp {
        Ok(res) => {
            println!("Booting... Server sent response: {:#?}", res);
            let addresses: Vec<String> = res
                .server
                .ip_addresses
                .ip_address
                .into_iter()
                .filter(|x| x.access == "public")
                .map(|x| x.address)
                .collect();
            println!("The IPs allocated were {:?}", addresses);
        }
        Err(inner) => panic!("Server creation failed! Inner {:#?}", inner),
    }
}
