use std::io::prelude::*;
use futures::future::join_all;
use tokio::task::JoinHandle;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use log::{info, error};
use clap::{Arg,App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("*** Alexa website checker ***")
                                .version("0.1")
                                .author("Arun <tomar.arun@gmail.com>")
                                .about("Checks access to alexa websites")
                            .arg(Arg::with_name("file")
                                .long("file")
                                .help("filename with the list of websites to check, default: websites.txt")
                                .takes_value(true))
                            .arg(Arg::with_name("retries")
                                .long("retries")
                                .help("number of retries")
                                .takes_value(true)
                            )
                            .get_matches();

    let path = matches.value_of("file").unwrap_or("websites.txt");
    let retries: u32 = matches.value_of("retires").unwrap_or("3").parse::<u32>().unwrap();
    // enable environment logger. use RUST_LOG=info to start logging
    env_logger::init();
    // read the list of websites from a file
    // let alexa_websites = get_websites().expect("something went wrong while getting the list of websites");
    let alexa_websites = get_websites_from_file(path).expect("something went wrong while getting the list of websites");    

    // Create a list of empty tasks
    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    // Connect to each website asynchronously in separate task
    for (id,site) in alexa_websites.iter().enumerate() {
        let url = site.clone();
        let site = site.to_owned();
        tasks.push(tokio::spawn(async move {
            // Build a client with custom user agent 
            // "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/60.0"
            let client = get_custom_client();
            // let mut counter = 0;
            match client.get(&url).send().await {
                Ok(resp) => {
                    // log the site & status
                    info!("ID: {} Success: url: {} - {:#?}", id, site, resp.status().as_str());
                },
                Err(e) => error!("ID: {} Fail: {} - {:#?}", id, url, e.to_string())
            } 
        }));
    }
        
    // Wait for all the tasks to finish
    join_all(tasks).await;
    Ok(())
}

fn get_custom_client() -> reqwest::Client {
    let user_agent = "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/60.0";
    let client = reqwest::Client::builder()
                                .user_agent(user_agent)
                                .timeout(Duration::from_secs(30))
                                .build();
    client.expect("issue creating a custom client")
}

fn get_websites_from_file(path: &str) -> Result<Vec<String>,Box<dyn std::error::Error>> {
    let file = File::open(path).expect(&format!("Couldn't open file {}", path));
    let buffer = BufReader::new(file);
    let mut websites: Vec<String> = Vec::new();
    for line in buffer.lines() {
        websites.push(line.expect("error reading line"));
    }
    Ok(websites)
}