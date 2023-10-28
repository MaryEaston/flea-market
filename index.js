// If you only use `npm` you can simply
// import { Chart } from "wasm-demo" and remove `setup` call from `bootstrap.js`.
class Chart {}

const canvas = document.getElementById("canvas");
const id = document.getElementById("id").textContent;
// const coord = document.getElementById("coord");
// const plotType = document.getElementById("plot-type");
// const pitch = document.getElementById("pitch");
// const yaw = document.getElementById("yaw");

let chart = null;

/** Main entry point */
export function main() {
  // while (!canvas) {} // canvasが出現するまで待機
  setupCanvas();
  loadMathJax();
}

/** This function is used in `bootstrap.js` to setup imports. */
export function setup(WasmChart) {
  Chart = WasmChart;
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
  const aspectRatio = canvas.width / canvas.height;
  const size = canvas.parentNode.offsetWidth;
  canvas.style.width = size + "px";
  canvas.style.height = size / aspectRatio + "px";
  canvas.width = size;
  canvas.height = size / aspectRatio;
  chart = Chart.power("canvas", Number(id));
}

function loadMathJax() {
  var script = document.createElement("script");
  script.id = "MathJax-script";
  script.src = "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg.js";
  script.async = true;
  document.getElementsByTagName("head")[0].appendChild(script);
}
