import express from "express";
import userRouter from "./routers/users.js";
import preferenceRouter from "./routers/preferences.js"
import swaggerJsdoc from "swagger-jsdoc";
import SwaggerUi from "swagger-ui-express";

const app = express();
app.use(express.json())

const PORT = process.env.PORT || 8080;

const options = {
  definition: {
    openapi: "3.0.0",
    info: {
      title: "Users API",
      version: "0.0.1",
    },
  },
  apis: ["./routers/*.js"],
};

const openapiSpecification = swaggerJsdoc(options);

app.use(userRouter);
app.use(preferenceRouter)
app.use("/docs", SwaggerUi.serve, SwaggerUi.setup(openapiSpecification));

app.listen(PORT, (err) => {
  if (err) {
    console.error(err);
  }
  console.log(`Server is running on ${PORT}`);
});
