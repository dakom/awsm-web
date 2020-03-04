//this will cause all the components to be registered
//note there is no module name
import "./components";

//this is possible to get the typescript without needing to go through the webpack dance
//but there might be a race condition, and anyway we don't need it just to get typing
//for one function ;)

//import * as _WasmCore from "../_static/wasm/demo/pkg/my_demo_bg";
//type WasmCore = typeof _WasmCore;

//see index.html
(window as any).load_wasm((wasm:any) => {
    wasm.run();
});