document.addEventListener("DOMContentLoaded", (_) => {
  document.querySelectorAll("code").forEach((el) => {
    const key = "file";
    const arrow = "👆";
    const regex = new RegExp(`${key}=\\S*`, "g");

    if (regex.test(el.className)) {
      const className = el.className.match(regex)[0];
      const fileName = className.split(`${key}=`).pop();
      const child = document.createElement("sub");
      child.style = "width: 100%; text-align: right; display: block;";
      child.innerHTML = `${arrow} ${fileName} ${arrow}`;
      el.parentNode.append(child);
    }
  });
});
