import { Vector3 } from "@react-three/fiber";
import { RigidBody } from "@react-three/rapier";
import React from "react";

const PI = Math.PI;

interface FloorProps {
    dimensions?: Vector3;
}

export function Floor(_: FloorProps): React.ReactElement {
    return (
        <RigidBody 
            args={[0, 0, 0]}
            colliders="cuboid"
            gravityScale={0}
            type="fixed"
        >
            <mesh rotation={[-PI/2, 0, 0]}>
                <meshStandardMaterial color="red"/>
                <boxGeometry
                    args={[40, 40, 0.1]}
                />
            </mesh>
        </RigidBody>
    );
}
