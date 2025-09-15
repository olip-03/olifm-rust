import init, {
  on_article_card_visible,
  on_article_card_click,
  on_tag_click,
} from "../pkg/web.js";
import "./article-observer.js";

async function run() {
  console.log("Starting WASM initialization...");
  try {
    await init();

    window.on_article_card_visible = on_article_card_visible;
    window.on_article_card_click = on_article_card_click;
    window.on_tag_click = on_tag_click;

    console.log("WASM initialized and functions exported globally");
  } catch (error) {
    console.error("Failed to initialize WASM module:", error);
  }
}

console.log("Loading script...");
run().catch(console.error);
