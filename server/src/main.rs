use std::time::Duration;

use anyhow::Result;
use simulation::{Instruction, PublicState, Simulation, SimulationContext, SimulationUpdate};
use tokio::{sync::{broadcast, mpsc}, time::interval};
use tracing::{error, info};

mod chat;
mod simulation;

#[tokio::main]
#[allow(unused)]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt().pretty().init();

    let (sim_tx, mut sim_rx) = broadcast::channel::<SimulationUpdate>(10);
    let (ins_tx, mut ins_rx) = mpsc::channel::<Instruction>(1000);
    let update_interval = Duration::from_millis(8);
    let instruction_interval = Duration::from_millis(200);
    
    //let test = tokio::spawn(async move {
    //    let mut int = interval(Duration::from_millis(160));
    //    int.tick().await;
    //    info!("sending");
    //    ins_tx.send(Instruction::Up).await;
    //});

    //let test_read = tokio::spawn(async move {
    //    let mut int = interval(Duration::from_millis(160));
    //    loop {
    //        let out = tokio::select! {
    //            out = sim_rx.recv() => Some(out),
    //            tick = int.tick() => None,
    //        };

    //        match out {
    //            Some(Ok(ref up)) => info!("{up:?}"),
    //            Some(Ok(SimulationUpdate::Done)) => {
    //                info!("Done");
    //                break;
    //            },
    //            Some(Err(e)) => {
    //                error!(err=%e, "simulation halted.");
    //                return;
    //            },
    //            None => {
    //                int.reset();
    //            }
    //        }
    //    }
    //});
    //let mut sim = Simulation::new(sim_tx, r_ins, up_interval);
    //sim.run().await;

    //tokio::join!(
    //    sim.run(), 
    //    test
    //);

    let mut sim = Simulation::new(
        sim_tx.clone(),
        update_interval,
        ins_rx,
        instruction_interval
    );
    let simulation_thread = tokio::spawn(async move {
        sim.run(&mut SimulationContext::default()).await
    });

    //let server_ctx = ServerContext {
    //    sim_channel: sim_tx.clone(),
    //    instruction_channel: ins_tx.clone(),
    //};

    //let server_thread = tokio::spawn(async move {
    //    ws::run_server(server_ctx, "0.0.0.0:6969");
    //});
    //
    //tokio::join!(
    //    simulation_thread,
    //    server_thread,
    //);

    Ok(())
}
