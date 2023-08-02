use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTP/2 client
    let client = Client::builder().http2_prior_knowledge().build()?;

    // Send the GET request
    let mut response = client.get("http://localhost:3000").send().await?;

    // Print response headers
    println!("Response headers:");
    for (name, value) in response.headers().iter() {
        println!("{}: {}", name.as_str(), value.to_str().unwrap_or(""));
    }

    // Print response body
    while let Some(chunk) = response.chunk().await? {
        println!("Chunk: {:?}", chunk);
    }

    // Print response body as text
    // let body_text = response.text().await?;
    // println!("Response body:\n{}", body_text);

    println!("Response headers:");
    for (name, value) in response.headers().iter() {
        println!("{}: {}", name.as_str(), value.to_str().unwrap_or(""));
    }

    // // Print response trailers if available
    // if let Some(trailers) = response
    //     .extensions()
    //     .get::<reqwest::hyper::header::Headers>()
    // {
    //     println!("Response trailers:");
    //     for (name, value) in trailers.iter() {
    //         println!("{}: {}", name.as_str(), value.to_str().unwrap_or(""));
    //     }
    // }

    Ok(())
}
