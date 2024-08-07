import React from "react";

export enum ControlCode {
    Jump,
    Up,
    Left,
    Right,
    Down,
    RotateCw,
    RotateCcw,
}

export function useControls(
    callback: (key: ControlCode) => void,
) {
    
    const onKeyDown = React.useCallback((ev: KeyboardEvent) => {
        switch(ev.code) {
            case "KeyW":
                callback(ControlCode.Up);
                break;
            case "KeyD":
                callback(ControlCode.Right);
                break;
            case "KeyA":
                callback(ControlCode.Left);
                break;
            case "KeyS":
                callback(ControlCode.Down);
                break;
            case "KeyE":
                callback(ControlCode.RotateCw);
                break;
            case "KeyQ":
                callback(ControlCode.RotateCcw);
                break;
            case "Space":
                callback(ControlCode.Jump);
                break;
            default:
                break;
        }
    }, [callback]);


    React.useEffect(() => {
        document.addEventListener("keydown", onKeyDown);
        return () => document.removeEventListener("keydown", onKeyDown);
    });
}
