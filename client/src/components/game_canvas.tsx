import React from "react";
import { Canvas, useFrame } from "@react-three/fiber";
import { Physics } from "@react-three/rapier";
import { OrbitControls } from "@react-three/drei";
import { Floor, Pawn } from "./game_components";
import { Suspense } from "react";
import { Coordinates, Orientation } from "../grpc-client/updates";
import { Euler, Vector3 } from "three";

export function GameCanvas(props: {
    coor: Coordinates,
    orient: Orientation,
}): React.ReactElement {
       
    return (
        <Canvas
            frameloop="demand"
            camera={{
                position: [props.coor.x, props.coor.y - 2, props.coor.z],
                type: "orthographic",
            }}
        >
        {/*<OrbitControls />*/}
            <ambientLight/>
            <Test {...props}/>
            {/*<Suspense>
                <Physics debug>
                    <Floor/>
                    <Pawn position={[0, 20, 0]}/>
                </Physics>
            </Suspense>*/}
        </Canvas>
    );
}


function Test(props: {
    coor: Coordinates,
    orient: Orientation,
}): React.ReactElement {
    useFrame(state => {
        state.camera.lookAt( new Vector3(...Object.values(props.coor)));
        state.camera.position.set(props.coor.x, props.coor.y -2, props.coor.z)
    });
    
    return (
        <mesh 
            position={new Vector3(...Object.values(props.coor))} 
            rotation={new Euler(props.orient.i, props.orient.j, props.orient.w)}
        >
            <boxGeometry/>
        </mesh>
    );
}
