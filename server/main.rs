// 1. Run this server:
//      ❯ cargo run -p server
//
// 2. Make a request and observe headers & trailers
//      ❯ curl -i http://127.0.0.1:3000/ --http2-prior-knowledge
//      HTTP/2 200
//      content-type: text/plain
//      date: Wed, 02 Aug 2023 10:01:36 GMT
//
//      Hello world!
//      x-attestation: foobar

use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Create the response body channel.
    let (mut sender, body) = Body::channel();

    // Send the response body in chunks asynchronously from a background task.
    tokio::spawn(async move {
        sender.send_data("Hello ".into()).await?;
        sender.send_data("world!\r\n".into()).await?;

        let mut trailers = hyper::header::HeaderMap::new();
        trailers.insert("X-Attestation", "foobar".parse().unwrap());
        sender.send_trailers(trailers).await?;
        Ok::<_, hyper::Error>(())
    });

    // Create the response with the given body.
    let response = Response::builder()
        .header("Content-Type", "text/plain")
        .body(body)
        .unwrap();

    Ok(response)
}

#[tokio::main]
async fn main() {
    // Bind the server to the specified IP address and port (127.0.0.1:3000).
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create a service to handle incoming requests.
    let make_service =
        make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handle_request)) });

    // Create a new hyper server with the service and bind it to the address.
    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    // Run the server indefinitely.
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
