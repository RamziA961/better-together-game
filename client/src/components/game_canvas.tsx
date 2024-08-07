import React from "react";
import { Canvas } from "@react-three/fiber";
import { Physics } from "@react-three/rapier";
import { OrbitControls } from "@react-three/drei";
import { Floor, Pawn } from "./game_components";
import { Suspense } from "react";

export function GameCanvas({}): React.ReactElement {
    return (
        <Canvas
            frameloop="demand"
            camera={{
                position: [0, 100, 0],
                type: "orthographic",
            }}
        >
            <OrbitControls />
            <ambientLight/>
            <Suspense>
                <Physics debug>
                    <Floor/>
                    <Pawn position={[0, 20, 0]}/>
                </Physics>
            </Suspense>
        </Canvas>
    );
}
