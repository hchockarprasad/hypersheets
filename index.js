const rust = import("./pkg/hypersheets");
setTimeout(() => {
  let container = document.getElementsByTagName("canvas")[0];
  container.addEventListener("test", (e) => {
    console.log(e);
  });
}, 5000);
rust
  .then((m) => {
    m.start();
  })
  .catch(console.error);
