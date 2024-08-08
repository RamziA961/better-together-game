use std::pin::Pin;

use tokio::sync::{broadcast, mpsc};
use tokio_stream::{Stream, StreamExt};
use tonic::{async_trait, Request, Response, Status};

use crate::{simulation::instruction, updates::{
    simulation_service_server::{SimulationService, SimulationServiceServer}, GenericRequest, GenericResponse, InstructionUpdate, SimulationUpdate
}};

#[derive(Debug, Clone)]
pub struct SimulationUpdateService {
    sim_tx: broadcast::Sender<SimulationUpdate>,
    ins_tx: mpsc::Sender<InstructionUpdate>,
}

impl SimulationUpdateService {
    pub fn new(
        sim_tx: broadcast::Sender<SimulationUpdate>,
        ins_tx: mpsc::Sender<InstructionUpdate>,
    ) -> Self {
        Self { sim_tx, ins_tx }
    }
}

#[async_trait]
impl SimulationService for SimulationUpdateService {
    type SubscribeToSimulationStream =
        Pin<Box<dyn Stream<Item = Result<SimulationUpdate, Status>> + Send + Sync + 'static>>;

    async fn subscribe_to_simulation(
        &self,
        request: Request<GenericRequest>,
    ) -> Result<Response<Self::SubscribeToSimulationStream>, Status> {
        let mut _incoming = request.into_inner();
        let mut sim_rx = self.sim_tx.subscribe();

        let outgoing = async_stream::try_stream! {
            while let Ok(update) = sim_rx.recv().await {
                yield update;
            }
        };

        Ok(Response::new(Box::pin(outgoing)))
    }

    async fn send_instruction(
        &self,
        instruction_req: Request<InstructionUpdate> 
    ) -> Result<Response<GenericResponse>, Status> {
        todo!();
    }
}
