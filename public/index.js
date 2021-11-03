import init, { parse } from "../pkg/markdown_parser.js";

const input = document.getElementById("input");
const output = document.getElementById("output");

init().then(() => {
  input.addEventListener("keyup", () => {
    output.innerHTML = parse(input.value);
  });
});
