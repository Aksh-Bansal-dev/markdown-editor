import init, { parse } from "./pkg/markdown_parser.js";

const input = document.getElementById("input");
const output = document.getElementById("output");

input.addEventListener("keydown", function (e) {
  if (e.key == "Tab") {
    e.preventDefault();
    var start = this.selectionStart;
    var end = this.selectionEnd;

    // Set textarea value to: text before caret + two spaces + text after caret
    this.value =
      this.value.substring(0, start) + "  " + this.value.substring(end);

    // Move caret past both inserted spaces 
    this.selectionStart = this.selectionEnd = start + 2;
  }
});

init().then(() => {
  input.addEventListener("keyup", () => {
    const renderedHtml = parse(input.value);
    const dangerousPatterns = [
      "<script",
      "javascript:",
      "data:",
      "vbscript:",
      "onerror=",
      "onload=",
      "onclick=",
      "onmouseover=",
    ];
    if (
      dangerousPatterns.some((pattern) =>
        renderedHtml.toLowerCase().includes(pattern),
      )
    ) {
      output.innerHTML = "Cannot render script tag!";
      return;
    }
    output.innerHTML = renderedHtml;
  });
});
