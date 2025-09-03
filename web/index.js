async function run() {
  console.log("Starting WASM initialization...");
  try {
    // Initialize the wasm module (no-modules target)
    console.log("Calling wasm_bindgen()...");
    await wasm_bindgen("./pkg/web_bg.wasm");
    console.log("WASM module initialized successfully!");
    // The wasm module's start function will run automatically after init
  } catch (error) {
    console.error("Failed to initialize WASM module:", error);
  }
}

console.log("Loading script...");
run().catch(console.error);
