import express from "express";
import cors from "cors";
import multer from "multer";

const app = express();
app.use(cors());
app.use(express.urlencoded({ extended: true }));

const storage = multer.diskStorage({
  destination: (req, file, cb) => {
    cb(null, "./uploads");
  },
  filename: (req, file, cb) => {
    const dateString = new Date().toISOString();
    console.log(file.originalname);
    cb(null, `${dateString}___${file.originalname}`);
  },
});

const upload = multer({ storage: storage });

app.post("/basicform", (req, res) => {
  console.log(req.body);
  res.send({ message: "ok" });
});

app.post("/fileupload", upload.single("file"), (req, res) => {
  console.log(req.file);
  res.send({ message: "ok" });
});

const PORT = process.env.APP_PORT || 8000;
app.listen(PORT, (req, res) => {
  console.log(`Server running on port ${PORT}`);
});
