// import init from "../pkg/web.js";

// article-observer.js
function setupArticleObserver() {
  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach(async (entry) => {
        if (entry.isIntersecting) {
          const card = entry.target;
          const cardId = card.dataset.cardId;
          const cardName = card.dataset.cardName;
          const cardPath = card.dataset.cardPath;

          if (window.on_article_card_visible) {
            console.log(cardId);
            console.log(cardName);
            console.log(cardPath);
            console.log("---");
            await window.on_article_card_visible(cardId, cardName, cardPath);
          } else {
            console.warn("on_article_card_visible not available yet");
          }

          observer.unobserve(card);
        }
      });
    },
    {
      threshold: 0.1,
      rootMargin: "10px",
    },
  );

  // Observe all existing article cards
  document.querySelectorAll(".base-card").forEach((card) => {
    observer.observe(card);
  });

  // Store observer globally for dynamic content
  window.articleCardObserver = observer;
}

// Auto-setup when DOM is ready
// if (document.readyState === "loading") {
//   document.addEventListener("DOMContentLoaded", setupArticleObserver);
// } else {
//   setupArticleObserver();
// }
