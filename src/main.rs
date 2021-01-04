
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // read the list of websites from a file

    let alexa_websites = vec![
        "http://google.com",
        "http://youtube.com",
        "http://amazon.com",
        "http://yahoo.com",
        "http://twitter.com",
        "http://live.com",
    ];

    // Create a list of empty tasks

    // Connect to each website asynchronously in separate task
    for site in alexa_websites {
        match reqwest::get(site).await {
            Ok(resp) => {
                println!("Site: {} - Status {:#?}", site, resp.status().as_str());
            },
            Err(_) => println!("Error accessing the site"),
        }
    }
    
    // Join all the tasks to the main thread
    
    Ok(())
}
