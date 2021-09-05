import init, { run_app } from "./pkg/bertrand.js";

async function main() {
  await init("/pkg/bertrand_bg.wasm");
  run_app();
}
main();

// After the page was rendered, execute the scripts.
// TODO
// For now, this doesn't make the <script src=""/> tags usable
// from later scripts.
document.addEventListener(
  "DOMNodeInserted",
  function (e) {
    if (e.target.id == "bertrand") {
      nodeScriptReplace(document.getElementById("bertrand"));
    }
  },
  false
);

function nodeScriptReplace(node) {
  if (node.tagName === "SCRIPT") {
    node.parentNode.replaceChild(nodeScriptClone(node), node);
  } else {
    var i = -1,
      children = node.childNodes;
    while (++i < children.length) {
      nodeScriptReplace(children[i]);
    }
  }

  return node;
}

function nodeScriptClone(node) {
  var script = document.createElement("script");
  script.text = node.innerHTML;

  var i = -1,
    attrs = node.attributes,
    attr;
  while (++i < attrs.length) {
    script.setAttribute((attr = attrs[i]).name, attr.value);
  }
  return script;
}
