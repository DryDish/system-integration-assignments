import {Router} from 'express'
const router = Router();

const preferencesArr = []


/**
 * @openapi
 * /preferences:
 *   post:
 *     description: add preferences to list.
 *     parameters:
 *       - in: body
 *         name: preferences
 *         required: true
 *         description: Numeric ID of the user to retrieve.
 *         schema:
 *           type: array
 *     responses:
 *       200:
 *         description: Returns updated array of preferences.
 */
router.post("/preferences", (req, res) => {
    const toAdd = req.body.preferences || [];
    preferencesArr.push(...toAdd) 
    res.send({presences: preferencesArr})
})


/**
 * @openapi
 * /preferences:
 *   get:
 *     description: Get preferences.
 *     responses:
 *       200:
 *         description: Returns an array preferences.
 */
router.get("/preferences", (req, res) => {
    res.send({presences: preferencesArr})
})

export default router;