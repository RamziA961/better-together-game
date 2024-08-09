import React from "react";
import { Canvas, useFrame } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei";
import { Coordinates, Orientation } from "../grpc-client/updates";
import { Euler, Quaternion, Vector3 } from "three";

export function GameCanvas(props: {
    coor: Coordinates,
    orient: Orientation,
}): React.ReactElement {
       
    return (
        <Canvas
            frameloop="demand"
            camera={{
                type: "orthographic",
            }}
        >
        <OrbitControls />
            <ambientLight/>
            <TestFloor/>
            <Test {...props}/>
        </Canvas>
    );
}


function Test(props: {
    coor: Coordinates,
    orient: Orientation,
}): React.ReactElement {
    //useFrame(state => {
    //    state.camera.lookAt( new Vector3(...Object.values(props.coor)));
    //    state.camera.position.set(props.coor.x, props.coor.y -2, props.coor.z)
    //});
    const position = new Vector3(...Object.values(props.coor));
    const quaternion = new Quaternion(...Object.values(props.orient));
    const euler = new Euler().setFromQuaternion(quaternion);
    const scale = new Vector3(10, 10, 10);
    console.log(props.coor);
    return (
        <mesh 
            position={position} 
            rotation={euler}
            scale={scale}
        >
            <meshStandardMaterial color={"blue"}/>
            <boxGeometry/>
        </mesh>
    );
}

function TestFloor(): React.ReactElement {
    return (
        <mesh 
            position={new Vector3(0, 0, 0)}
            scale={new Vector3(100, 0, 100)} 
        >
            <meshStandardMaterial color={"green"}/>
            <boxGeometry/>
        </mesh>
    );
}

