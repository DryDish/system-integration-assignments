from datetime import timezone
import json
from typing_extensions import Self
from fastapi import FastAPI
from requests.exceptions import ConnectionError

import uvicorn
import datetime
import requests


app = FastAPI()


class DateTimeResponse:
    utc_time_stamp: str

    def __init__(self, timestamp: str) -> Self:
        self.utc_time_stamp = timestamp


class HttpResponse:
    status: int
    message: str

    def __init__(self, status, message) -> Self:
        self.status = status
        self.message = message

    def __repr__(self) -> str:
        return f'"status": {self.status}, "message": {self.message}'

    def internal_server_error(self) -> Self:
        self.status = 500
        self.message = "Internal Server Error"


@app.get("/datetime")
def date_time():
    time = datetime.datetime.now(timezone.utc)

    return {"utc_time_stamp":  time.isoformat()}


@app.get("/datetime_from_rust")
def date_time_from_rust():
    try:
        response = requests.get("http://localhost:5000/datetime")
    except ConnectionError:
        return HttpResponse(500, "Other Server is unreachable.")

    if response.status_code == 200:
        return DateTimeResponse(response.json()["utc_time_stamp"])
    elif response.status_code == 500:
        return HttpResponse.internal_server_error()
    else:
        return HttpResponse(418, "Unknown error.")


if __name__ == '__main__':
    port = 3000
    host = '127.0.0.1'
    uvicorn.run(
        "main:app",
        port=port,
        host=host,
        reload=True,
    )
    print(f"Server running on port {port} on host {host}")
