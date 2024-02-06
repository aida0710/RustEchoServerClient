mod server;
mod client;

fn main() {
    let server_handle = std::thread::spawn(|| {
        if let Err(e) = server::start_server() {
            eprintln!("Server error: {}", e);
        }
    });

    std::thread::sleep(std::time::Duration::from_secs(1));

    let client_handle = std::thread::spawn(|| {
        if let Err(e) = client::send_message() {
            eprintln!("Client error: {}", e);
        }
    });

    server_handle.join().unwrap();
    client_handle.join().unwrap();
}