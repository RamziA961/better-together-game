import * as React from "react";
import { GameCanvas } from "./components/game_canvas";
import { SimulationServiceClient } from "./grpc-client/updates.client";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { Coordinates, Orientation } from "./grpc-client/updates";
import { Euler, Vector3 } from "three";

function App(): React.ReactElement {
    const [coor, setCoor] = React.useState<Coordinates>({
        x: 0,
        y: 0,
        z: 0,
    });
    const [orient, setOrient] = React.useState<Orientation>({
        i: 0,
        j: 0,
        k: 0,
        w: 0,
    });

    const simulationService = React.useMemo(
        () => {
            const transport = new GrpcWebFetchTransport({
                format: "text",
                baseUrl: "http://0.0.0.0:6969",
            });
            return new SimulationServiceClient(transport);
    }, []);
    
    React.useEffect(() => {
        const subscribe = async () => {
            for await (let resp of simulationService.subscribeToSimulation({ ok: true }).responses) {
                if (resp.spatialUpdates.length === 0) {
                    continue;
                }

                const spatial = resp.spatialUpdates[0];
                if(spatial.coordinates && spatial.orientation) {
                    setCoor(spatial.coordinates);
                    setOrient(spatial.orientation);
                }
            }
        }
        subscribe();
    }, [simulationService]);

    return (
        <div id="app-main">
           <GameCanvas coor={coor} orient={orient}/>
        </div>
    );
}

export default App;
