import express from "express";
const app = express();
const port = 8080;

app.use(express.static("public"));

app.get("/", (req, res) => {
  res.sendFile(__dirname + "/public/index.html");
});

app.get("/synchronizeTime", (req, res) => {
  res.writeHead(200, {
    "Content-Type": "text/event-stream",
    "Cache-Control": "no-cache",
    Connection: "keep-alive",
  });
  setInterval(() => {
    res.write(`data: ${new Date()} \n\n`);
  }, 1000);
});

app.listen(port, () => {
  console.log("Server running on port", port);
});
