from fastapi import FastAPI

import requests

app = FastAPI()

@app.get("/")
def _():
    response = requests.get("http://localhost:3000/")
    print(response.content)
    return {"Hello": "World, Python"}

@app.get("/b")
def _():
    return {"Endpoint": True }