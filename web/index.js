import init from "./pkg/web.js";

async function run() {
  console.log("Starting WASM initialization...");
  try {
    // Initialize the wasm module
    console.log("Calling init()...");
    wasm = await init();
    console.log("WASM module initialized successfully!");
    // The wasm module's start function will run automatically after init
  } catch (error) {
    console.error("Failed to initialize WASM module:", error);
  }
}

console.log("Loading script...");
run().catch(console.error);
