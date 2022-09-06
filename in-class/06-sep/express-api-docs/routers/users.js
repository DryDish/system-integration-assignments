import { Router } from "express";
const router = Router();

/**
 * @openapi
 * /users:
 *   get:
 *     description: Get all users.
 *     responses:
 *       200:
 *         description: Returns an array users.
 */
router.get("/users", (req, res) => {
  res.send({ users: [] });
});

export default router;
