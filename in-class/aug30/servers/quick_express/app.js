import express from "express";
import path from 'path';

const app = express();

app.use(express.static("public"))

app.get("/", (req, res) => {
  res.send({hello: "world, Express"});
});


app.get("/python", async (req, res) => {
  const something = await fetch("http://127.0.0.1:8000/")
  res.send({body: await something.json()})
})

app.get("/duck", (req, res) => {
  res.sendFile(path.resolve("./public/duck.html"));
});

app.listen(3000, () => {
  console.log("Server is running on", 3000);
});
  