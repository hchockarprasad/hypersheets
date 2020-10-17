const rust = import("./pkg/hypersheets");
rust
  .then((m) => {
    m.start();
  })
  .catch(console.error);
