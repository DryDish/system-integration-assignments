from datetime import timezone
from fastapi import FastAPI

import uvicorn
import datetime


app = FastAPI()


@app.get("/")
def default():
    return {"hello": "world"}


@app.get("/items/{item_id}")
def semi_default(item_id: str):
    return {"message": f"hello {item_id}"}


if __name__ == '__main__':
    port = 8080
    host = '127.0.0.1'
    uvicorn.run(
        "main:app",
        port=port,
        host=host,
        reload=True,
    )
    print(f"Server running on port {port} on host {host}")
