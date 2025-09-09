import init, { on_article_card_visible } from "../pkg/web.js";
import "./article-observer.js";

window.on_article_card_visible = on_article_card_visible; // export to global

async function run() {
  console.log("Starting WASM initialization...");
  try {
    await init();
  } catch (error) {
    console.error("Failed to initialize WASM module:", error);
  }
}

console.log("Loading script...");
run().catch(console.error);
