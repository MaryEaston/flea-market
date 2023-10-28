import rust_init from "./pkg/package.js";

rust_init("./pkg/package_bg.wasm");

async function init() {
  // while (!canvas) {} // canvasが出現するまで待機
  if (typeof process == "object") {
    // We run in the npm/webpack environment.
    const [{ Chart }, { main, setup }] = await Promise.all([
      import("package"),
      import("./index.js"),
    ]);
    setup(Chart);
    main();
  } else {
    const [{ Chart, default: init }, { main, setup }] = await Promise.all([
      import("./pkg/package.js"),
      import("./index.js"),
    ]);
    console.log("a");
    await init();
    // await init("/pkg/package_bg.wasm");
    console.log("b");
    setup(Chart);
    main();
  }
}

// 変更を監視するノードを選択
const targetNode = document.getElementById("app");

// (変更を監視する) オブザーバーのオプション
const config = { attributes: true, childList: true, subtree: true };

// 変更が発見されたときに実行されるコールバック関数
const callback = function (mutationsList, observer) {
  init();
  observer.disconnect();
};

// コールバック関数に結びつけられたオブザーバーのインスタンスを生成
const observer = new MutationObserver(callback);

// 対象ノードの設定された変更の監視を開始
observer.observe(targetNode, config);
