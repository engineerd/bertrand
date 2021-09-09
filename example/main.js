import init, { run_app } from "./pkg/bertrand.js";

async function main() {
  await init("/pkg/bertrand_bg.wasm");
  run_app();
}
main();

// The Wasm module injects an element with `id=bertrand`
// with the rendered page.
//
// Because this is dynamically added, and the page might contain
// scripts, we want to load them as scripts and execute them.
// TODO:
// This loads scripts from source (`<script src=""/>`), but
// they are unusable from later scripts. Until this is fixed,
// source scripts should be placed in the main `index.html`.
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
    node.parentNode.replaceChild(cloneAsScript(node), node);
  } else {
    var i = -1,
      children = node.childNodes;
    while (++i < children.length) {
      nodeScriptReplace(children[i]);
    }
  }

  return node;
}

function cloneAsScript(node) {
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
