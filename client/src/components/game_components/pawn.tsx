import * as React from "react";
import { RapierRigidBody, RigidBody } from "@react-three/rapier";
import { ControlCode, useControls } from "./use_controls";
import { useFrame } from "@react-three/fiber";

const MAX_LIN_SPEED = 10;
const MAX_ANG_SPEED = 4 * Math.PI;
const LIN_FACTOR = 150;
const ANG_FACTOR = 150;

interface PawnProps {
    position: [number, number, number];
    mass?: number;
}

export function Pawn({
    position,
    mass,
}: PawnProps): React.ReactElement {
    const rigidBodyRef = React.useRef<RapierRigidBody>(null);
    const [grounded, setGrounded] = React.useState(false);

    const deltaT = React.useRef<number>(1);

    useFrame((_, delta) => {
        deltaT.current = delta;
    });

    useControls((control: ControlCode) => {
        if(!rigidBodyRef.current) {
            return;
        }

        const { current } = rigidBodyRef;
        const { current: delta } = deltaT;

        switch(control) {
            case ControlCode.Jump:
                if(grounded) {
                    current.applyImpulse({ x: 0, y: 7, z: 0}, true);
                }
                break;
            case ControlCode.Left:
                if(current.linvel().x >= -MAX_LIN_SPEED && grounded) {
                    current.applyImpulse({ x: -delta * LIN_FACTOR, y: 0, z: 0}, true);
                }
                break;
            case ControlCode.Right:
                if(current.linvel().x <= MAX_LIN_SPEED && grounded) {
                    current.applyImpulse({ x: delta * LIN_FACTOR, y: 0, z: 0}, true);
                }
                break;
            case ControlCode.Up:
                if(current.linvel().z >= -MAX_LIN_SPEED && grounded) {
                    current.applyImpulse({ x: 0, y: 0, z: -delta * LIN_FACTOR}, true);
                }
                break;
            case ControlCode.Down:
                if(current.linvel().z <= MAX_LIN_SPEED && grounded) {
                    current.applyImpulse({ x: 0, y: 0, z: delta * LIN_FACTOR}, true);
                }
                break;
            case ControlCode.RotateCw:
                if(current.angvel().y >= -MAX_ANG_SPEED) {
                    current.applyTorqueImpulse({ x: 0, y: -Math.PI * delta * ANG_FACTOR, z: 0 }, true);
                }
                break;
            case ControlCode.RotateCcw:
                if(current.angvel().y <= MAX_ANG_SPEED) {
                    current.applyTorqueImpulse({ x: 0, y: Math.PI * delta * ANG_FACTOR, z: 0 }, true);
                }
                break;
            default:
                break;
        }
    });

    return (
        <RigidBody
            ref={rigidBodyRef}
            colliders="cuboid"
            args={[0, 0, 0]}
            position={position}
            mass={mass}
            onCollisionEnter={() => setGrounded(true)}
            onCollisionExit={() => setGrounded(false)}
        >
            <mesh>
                <boxGeometry/>
            </mesh>
        </RigidBody>
    );
}
