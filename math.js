document.addEventListener("DOMContentLoaded", function () {
  // この中の最後の方に以下をかく
  (function () {
    window.MathJax = {
      startup: {
        pageReady: function () {
          let observer = new MutationObserver(function () {
            MathJax.texReset();
            MathJax.typesetPromise(targetElement.childNodes);
          });
          observer.observe(targetElement, { childList: true });
          return MathJax.startup.defaultPageReady();
        },
      },
      tex: {
        inlineMath: [
          ["$", "$"],
          ["\\(", "\\)"],
        ],
      },
      svg: {
        fontCache: "global",
      },
    };
    var scriptIE = document.createElement("script");
    scriptIE.src = "https://polyfill.io/v3/polyfill.min.js?features=es6";
    scriptIE.async = false;
    document.getElementsByTagName("head")[0].appendChild(scriptIE);
    var script = document.createElement("script");
    script.src = "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg.js";
    script.async = true;
    document.getElementsByTagName("head")[0].appendChild(script);
  })();
});
