use std::{pin::Pin, time::Duration};

use tokio::{sync::{broadcast, mpsc}, time};
use tokio_stream::{Stream, StreamExt};
use tonic::{async_trait, Request, Response, Status};
use tracing::{error, info, instrument};

use crate::{
    simulation::instruction,
    updates::{
        simulation_service_server::{SimulationService, SimulationServiceServer},
        GenericRequest, GenericResponse, InstructionUpdate, SimulationUpdate,
    },
};

#[derive(Debug)]
pub struct SimulationUpdateService {
    sim_rx: broadcast::Receiver<SimulationUpdate>,
    ins_tx: mpsc::Sender<InstructionUpdate>,
}

impl SimulationUpdateService {
    pub fn new(
        sim_rx: broadcast::Receiver<SimulationUpdate>,
        ins_tx: mpsc::Sender<InstructionUpdate>,
    ) -> Self {
        Self { sim_rx, ins_tx }
    }
}

#[async_trait]
impl SimulationService for SimulationUpdateService {
    type SubscribeToSimulationStream =
        Pin<Box<dyn Stream<Item = Result<SimulationUpdate, Status>> + Send + Sync + 'static>>;

    #[instrument(skip_all)]
    async fn subscribe_to_simulation(
        &self,
        request: Request<GenericRequest>,
    ) -> Result<Response<Self::SubscribeToSimulationStream>, Status> {
        info!("New subscriber");
        let mut sim_rx1 = self.sim_rx.resubscribe();

        let outgoing = async_stream::try_stream! {
            while let Ok(update) = sim_rx1.recv().await {
                yield update;
            }
        };

        Ok(Response::new(Box::pin(outgoing)))
    }

    async fn send_instruction(
        &self,
        instruction_req: Request<InstructionUpdate>,
    ) -> Result<Response<GenericResponse>, Status> {
        todo!();
    }
}
