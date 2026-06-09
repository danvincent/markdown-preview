use std::thread;
use std::net::TcpStream;
use tiny_http::{Server, Response, Header};
use md_viewer::render_markdown;

#[test]
fn test_serve_once_then_quit_html() {
    // Setup: create a short HTML to serve
    let html = render_markdown("Hello **test** world!", "Test");
    // Bind to any available port
    let server = Server::http("127.0.0.1:0").expect("Cannot start server");
    let addr = server.server_addr();
    let html = std::sync::Arc::new(html);
    let served_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let served_flag_2 = served_flag.clone();

    let handle = thread::spawn(move || {
        for request in server.incoming_requests() {
            let response = Response::from_string(html.as_str())
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
            let _ = request.respond(response);
            served_flag_2.store(true, std::sync::atomic::Ordering::SeqCst);
            break;
        }
    });

    // Try to connect as client
    let url = format!("http://{}", addr);
    let resp = ureq::get(&url).call().unwrap();
    assert_eq!(resp.status(), 200);
    let body = resp.into_string().unwrap();
    assert!(body.contains("<strong>test</strong>"));
    
    // Wait server thread to finish
    handle.join().unwrap();
    assert!(served_flag.load(std::sync::atomic::Ordering::SeqCst));
    // Try again: should fail (server closed)
    use std::net::{ToSocketAddrs};
    let sock_addr = addr.to_string().to_socket_addrs().unwrap().next().unwrap();
    assert!(TcpStream::connect(sock_addr).is_err());
}
