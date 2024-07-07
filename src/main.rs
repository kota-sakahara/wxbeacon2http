use std::error::Error;
use std::net::SocketAddr;
use wxbeacon2http_lib::{ble, server, SharedState};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "wxbeacon2http")]
struct Opt {
    #[structopt(short, long, default_value = "127.0.0.1")]
    ip: String,

    #[structopt(short, long, default_value = "3030")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let addr: SocketAddr = format!("{}:{}", opt.ip, opt.port).parse()?;

    let state = SharedState::default();
    let server_state = state.clone();
    let ble_state = state.clone();

    let server_task = tokio::spawn(async move {
        if let Err(e) = server::run_server(server_state, addr).await {
            eprintln!("Server error: {}", e);
        }
    });

    let ble_task = tokio::spawn(async move {
        if let Err(e) = ble::run_ble_scan(ble_state).await {
            eprintln!("BLE scan error: {}", e);
        }
    });

    tokio::try_join!(server_task, ble_task)?;

    Ok(())
}
