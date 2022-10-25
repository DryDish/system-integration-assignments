from fileinput import filename
from fastapi import FastAPI, File, UploadFile, Form
from fastapi.middleware.cors import CORSMiddleware

import uvicorn

app = FastAPI()


app.add_middleware(
    CORSMiddleware,
    allow_origins=['*'],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.post("/basicform")
async def basicform(username: str = Form(...), password: str = Form(default=..., min_length=8)):
    return (username, password)


@app.post("/fileupload")
async def fileupload(file: UploadFile):
    print(file)
    return {'filename': file.filename}


if __name__ == '__main__':
    port = 8000
    host = '127.0.0.1'
    uvicorn.run(
        "main:app",
        port=port,
        host=host,
        reload=True,
    )
    print(f"Server running on port {port} on host {host}")
