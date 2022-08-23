const express = require("express");
const app = express();

app.get("/endpoint", (req, res) => {
  res.send({ message: "OK" });
});

app.listen(3000, () => {
  console.log("Server running on", 3000);
});
