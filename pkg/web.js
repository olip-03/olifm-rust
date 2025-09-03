import * as wasm from "./web_bg.wasm";
export * from "./web_bg.js";
import { __wbg_set_wasm } from "./web_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
