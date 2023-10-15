use common_library_rust::socket::client::Client;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

fn job() -> Result<(), String> {
    let address = match SocketAddr::from_str(&env::args().collect::<Vec<_>>()[1]) {
        Ok(address) => address,
        Err(e) => return Err(format!("{}", e)),
    };

    let mut client = Client::new();

    match client.connect(address, Duration::new(3, 0)) {
        Ok(_) => (),
        Err(e) => return Err(format!("{}", e)),
    }

    match client.read(1024) {
        Ok(data) => println!("read data : ({})", data),
        Err(e) => return Err(format!("{}", e)),
    };

    let data = String::from("test\r\n");
    match client.write(&data) {
        Ok(_) => println!("write data : ({})", data),
        Err(e) => return Err(format!("{}", e)),
    };

    Ok(())
}

fn main() {
    println!("process start");

    match job() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    println!("process end");
}
