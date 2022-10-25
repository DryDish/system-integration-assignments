from fastapi import FastAPI, Request
import json
import uvicorn

app = FastAPI()


@app.post("/webhook")
async def default(request: Request):
    print(json.loads(await request.body()))
    return {"response": "zawardo"}


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
