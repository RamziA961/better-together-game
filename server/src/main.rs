use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use anyhow::Result;
use service::simulation_service;
use simulation::{Simulation, SimulationContext};
use tokio::sync::{broadcast, mpsc};
use tonic::transport::Server;
use tower_http::cors;
use tracing::{error, info};

mod chat;
mod service;
mod simulation;

pub mod updates {
    tonic::include_proto!("updates");
}

use updates::{
    simulation_service_server::SimulationServiceServer, InstructionUpdate, SimulationUpdate,
};

#[tokio::main]
#[allow(unused)]
async fn main() -> Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 6969);

    let _ = tracing_subscriber::fmt().pretty().init();

    let (sim_tx, mut sim_rx) = broadcast::channel::<SimulationUpdate>(10);
    let (ins_tx, mut ins_rx) = mpsc::channel::<InstructionUpdate>(1000);
    let update_interval = Duration::from_millis(8);
    let instruction_interval = Duration::from_millis(200);

    let mut sim = Simulation::new(
        sim_tx.clone(),
        update_interval,
        ins_rx,
        instruction_interval,
    );
    let simulation_thread = tokio::spawn(async move {
        info!("Starting simulation thread");
        sim.run(&mut SimulationContext::default()).await
    });

    let sim_up_svc = simulation_service::SimulationUpdateService::new(sim_rx, ins_tx);
    let sim_up_server = SimulationServiceServer::new(sim_up_svc);

    let cors = cors::CorsLayer::new().allow_origin(cors::Any);

    let server = Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(sim_up_server))
        .serve(addr);

    info!("Starting server at {addr}");
    tokio::join!(simulation_thread, server);
    Ok(())
}
