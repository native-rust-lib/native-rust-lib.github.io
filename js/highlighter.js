document.addEventListener("DOMContentLoaded", (_) => {
  document.querySelectorAll("code").forEach((el) => {
    if (el.className.includes("#")) {
      const expression = el.className.split(" ").find((x) => x.includes("#"));
      const htmlLines = el.innerHTML.split("\n");

      const lineNumbers = expression.substring(1, expression.length);
      const lines = lineNumbers
        .split(";")
        .map((x) => {
          if (x.includes("-")) {
            const [lower, upper] = x.split("-").map((x) => parseInt(x));
            return Array.from(
              new Array(upper - lower + 1),
              (_, i) => i + lower - 1
            );
          }
          return parseInt(x) - 1;
        })
        .flat();

      const newHtmlLines = htmlLines.map((x, i) =>
        lines.includes(i) ? `<mark>${x}</mark>` : x
      );

      el.innerHTML = newHtmlLines.join("\n");
    }
  });
});
