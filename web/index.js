import init from "./pkg/web.js";

async function run() {
  try {
    await init();
  } catch (error) {
    console.error("Failed to initialize WASM module:", error);
  }
}
run().catch(console.error);
