use warp::Filter;
use std::error::Error;
use std::net::SocketAddr;

use crate::SharedState;

pub async fn run_server(state: SharedState, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let routes = warp::path("sensor")
        .and(with_state(state.clone()))
        .and_then(handle_sensor_request)
        .or(warp::path("status").and(with_state(state)).and_then(handle_status_request));

    println!("status: http://{}/status", addr);
    println!("sensor: http://{}/sensor", addr);
    warp::serve(routes).run(addr).await;

    Ok(())
}

async fn handle_status_request(state: SharedState) -> Result<impl warp::Reply, warp::Rejection> {
    let data = state.read().await;
    match &*data {
        Some(_) => Ok(warp::reply::json(&"available")),
        None => Ok(warp::reply::json(&"not_yet")),
    }
}

async fn handle_sensor_request(state: SharedState) -> Result<impl warp::Reply, warp::Rejection> {
    let data = state.read().await;
    match &*data {
        Some(env_datum) => Ok(warp::reply::json(env_datum)),
        None => Err(warp::reject::not_found()),
    }
}

fn with_state(state: SharedState) -> impl Filter<Extract = (SharedState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
