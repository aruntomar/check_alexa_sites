use futures::future::join_all;
use tokio::task::JoinHandle;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // read the list of websites from a file
    let alexa_websites = get_websites().expect("something went wrong while getting the list of websites");
        

    // Create a list of empty tasks
    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    // Connect to each website asynchronously in separate task
    for site in alexa_websites {
        let url = site.clone();
        tasks.push(tokio::spawn(async move {
            // Build a client with custom user agent 
            // "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/60.0"
            let client = get_custom_client();    
            match client.get(url).send().await {
                Ok(resp) => {
                    // log the site & status
                    println!("Site: {} - Status {:#?}", site, resp.status().as_str());
                },
                Err(_) => println!("Error accessing the site {}", url),
            } 
        }));
    }
        
    // Wait for all the tasks to finish
    join_all(tasks).await;
    Ok(())
}

fn get_websites() -> Result<Vec<&'static str>,Box<dyn std::error::Error>> {
    let websites = vec![
        "http://localhost",
        "http://google.com",
        "http://youtube.com",
        "http://google.ca",
        "http://facebook.com",
        "http://www.reddit.com",
        "http://amazon.ca",
        "http://wikipedia.org",
        "http://twitter.com",
        "http://netflix.com",
        "http://live.com",
        "http://yahoo.com",
        "http://instagram.com",
        "http://amazon.com",
        "http://twitch.tv",
        "http://kijiji.ca",
        "http://pornhub.com",
        "http://linkedin.com",
        "https://www.td.com",
        "http://www.royalbank.com",
        "http://imgur.com",
        "http://login.microsoftonline.com",
        "http://livejasmin.com",
        "http://bestbuy.ca",
        "http://imdb.com",
        "http://office.com",
        "http://www.cbc.ca",
        "http://T.co",
        "http://wikia.com",
        "http://theweathernetwork.com",
        "http://ebay.ca",
        "http://narcity.com",
        "http://scotiabank.com",
        "http://vice.com",
        "http://www.rbcroyalbank.com",
        "http://tumblr.com",
        "http://paypal.com",
        "http://www.walmart.ca",
        "http://pinterest.ca",
        "http://microsoft.com",
        "http://apple.com",
        "http://cnn.com",
        "http://msn.com",
        "http://canada.ca",
        "http://xvideos.com",
        "http://bing.com",
        "http://redflagdeals.com",
        "http://quora.com",
        "http://github.com",
        "http://blogspot.com",
        "http://ebay.com",
    ];
    Ok(websites)
}

fn get_custom_client() -> reqwest::Client {
    let user_agent = "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/60.0";
    let client = reqwest::Client::builder()
                                .user_agent(user_agent)
                                .build();
    client.expect("issue creating a custom client")
}