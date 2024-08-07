import React from "react";
import { useFrame } from "@react-three/fiber";

export function AxesHelper(props: { size: number }): React.ReactElement {
    const [dims, setDims] = React.useState<[number, number]>([0, 0]);

    useFrame(({ viewport, camera }) => {
        console.log(camera.position);
        const { width, height } = viewport.getCurrentViewport();
        console.log(height, width);
        console.log("-----------");
        setDims([camera.position.z + height / 2, camera.position.x - width / 2]);
    });
    
    return (
        <axesHelper
            args={[props.size]}
            position={[
                dims[1],
                0, 
                dims[0],
            ]}
        />
    );
}
