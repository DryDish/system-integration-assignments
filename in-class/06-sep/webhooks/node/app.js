import express from "express";
const app = express();

const port = process.env.PORT || 8080;

app.use(express.json())


app.post("/webhook", (req, res) => {
    console.log(req.body);
    res.send();
})


app.listen(port,() => {
    console.log(`Server is running on port: ${port}`);
})