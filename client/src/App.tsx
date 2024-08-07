import * as React from "react";
import { GameCanvas } from "./components/game_canvas";

function App(): React.ReactElement {
 
    return (
        <div id="app-main">
            <GameCanvas/>
        </div>
    );
}

export default App;
