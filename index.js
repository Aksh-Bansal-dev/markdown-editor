import init, { parse } from "./pkg/markdown_parser.js";

const input = document.getElementById("input");
const output = document.getElementById("output");

input.addEventListener("keydown", function (e) {
  if (e.key == "Tab") {
    e.preventDefault();
    var start = this.selectionStart;
    var end = this.selectionEnd;

    // set textarea value to: text before caret + tab + text after caret
    this.value =
      this.value.substring(0, start) + "\t" + this.value.substring(end);

    // put caret at right position again
    this.selectionStart = this.selectionEnd = start + 1;
  }
});

init().then(() => {
  input.addEventListener("keyup", () => {
    const a = parse(input.value);
    const b = "<script>";
    for (let i = 0; i < a.length; i++) {
      let j = 0;
      let ii = i;
      while (a[ii] == b[j]) {
        j++;
        ii++;
      }
      if (j === b.length) {
        output.innerHTML = "Cannot render script tag!";
        break;
      }
    }
    // const res = parse(input.value);
    // if (res.indexOf("<script>") > -1) {
    //   output.innerHTML = "Cannot render script tag!";
    //   return;
    // }
    output.innerHTML = res;
  });
});
