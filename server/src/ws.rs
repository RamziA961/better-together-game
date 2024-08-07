use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use anyhow::{anyhow, Result};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    select,
    sync::{broadcast, mpsc, Mutex},
};
use tokio_tungstenite::tungstenite::Message;
use tracing::warn;

use crate::simulation::{Instruction, SimulationUpdate};

type Tx = mpsc::UnboundedSender<Payload>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Payload {
    // Send + Recv
    ChatMessage(String),

    // Send
    SimulationUpdate(SimulationUpdate),

    // Recv
    SimulationInstruction(Instruction),

    NoPayload
}

enum Action {
    Send(Payload),
    Recv(Payload),
    Noop,
    Close,
}

#[derive(Debug, Clone)]
pub struct ServerContext {
    pub sim_channel: broadcast::Sender<SimulationUpdate>,
    pub instruction_channel: mpsc::Sender<Instruction>,
}

pub async fn run_server(ctx: ServerContext, addr: impl ToSocketAddrs) -> Result<()> {
    let peer_map = PeerMap::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind(&addr).await?;

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(client_handler(ctx.clone(), peer_map.clone(), stream, addr));
    }

    Ok(())
}

pub async fn client_handler(
    ctx: ServerContext,
    peer_map: PeerMap,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<()> {
    let ws_stream = tokio_tungstenite::accept_async(stream).await?;
    let (ws_tx, mut ws_rx) = mpsc::unbounded_channel();
    let mut sim_update_rx = ctx.sim_channel.subscribe();
    let ins_tx = ctx.instruction_channel;

    peer_map.lock()
        .await
        .insert(addr, ws_tx.clone());

    let (mut outgoing, mut incoming) = ws_stream.split();

    loop {
        let res: Result<Action> = select! {
            sim_up = sim_update_rx.recv() => {
                sim_up
                    .map(|up| Action::Send(Payload::SimulationUpdate(up)))
                    .map_err(|e| anyhow!("{e:?}"))
            },
            recvd = incoming.next() => {
                let payload = match recvd {
                    Some(Ok(Message::Text(s))) => {
                        match s.chars().next() {
                            Some('I') => {
                                let target = s.chars().skip_while(|c| c != &'{').collect::<String>();
                                let deser = serde_json::from_str::<Instruction>(&target);
                                deser      
                                    .map(|ins| Payload::SimulationInstruction(ins))
                                    .map_err(|e| {
                                        warn!(err=?e, "could not parse {s}");
                                        anyhow!("{e:?}")
                                    })
                                    .ok()
                            },
                            _ => Some(Payload::NoPayload),
                        }
                    },
                    v => {
                        warn!("Unknown {v:?}");
                        None
                    },
                };

                match payload {
                    Some(payload) if matches!(payload, Payload::SimulationInstruction(_)) => Ok(Action::Recv(payload)),
                    Some(payload) if matches!(payload, Payload::ChatMessage(_)) => Ok(Action::Recv(payload)),
                    Some(Payload::NoPayload) => Ok(Action::Noop),
                    None => Ok(Action::Close),
                    _ => Err(anyhow!("Unknown"))
                }
            },
            else => { break }
        };
    
        if res.is_err() {
            return res.map(|_| ());
        }

        match res.unwrap() {
            Action::Send(payload) if matches!(payload, Payload::SimulationUpdate(_)) => {
                let ser = serde_json::to_string(&payload)?;
                outgoing.send(Message::Text(ser)).await?;
            },
            Action::Recv(Payload::ChatMessage(_s)) => {
                //incoming here
                unimplemented!()
            },
            Action::Recv(Payload::SimulationInstruction(instruction)) => {
                ins_tx.send(instruction).await?;
            },
            Action::Noop => continue,
            Action::Close => break,
            _ => return Err(anyhow!("Invalid action")),
        }
    }

    Ok(())
}
