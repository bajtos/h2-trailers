use hyper::body::HttpBody;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create an HTTP/2 client with support for HTTPS
    let https = HttpsConnector::new();
    let client: Client<HttpsConnector<HttpConnector>> =
        Client::builder().http2_only(true).build(https);

    let uri = "http://localhost:3000".parse()?;

    // Send the GET request and await the response
    let response = client.get(uri).await?;

    println!("Response status: {}", response.status());
    // Print response headers
    println!("Response headers:");
    for (name, value) in response.headers() {
        println!("  {}: {}", name, value.to_str()?);
    }

    // Collect and print response body as text
    let mut body = response.into_body();

    let mut body_bytes = Vec::<u8>::with_capacity(1024);
    while let Some(buf) = body.data().await {
        body_bytes.extend_from_slice(buf?.as_ref());
    }
    let body_text = String::from_utf8_lossy(&body_bytes);
    println!("Response body:\n{}", body_text);

    println!("Response trailers:");
    if let Some(trailers) = body.trailers().await? {
        for (name, value) in trailers {
            println!("  {:?}: {}", name, value.to_str()?);
        }
    }

    Ok(())
}
