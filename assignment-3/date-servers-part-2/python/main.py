from datetime import timezone
from fastapi import FastAPI

import uvicorn
import datetime

app = FastAPI()


@app.get("/datetime")
def date_time():
    time = datetime.datetime.now(timezone.utc)

    return {"utc_time_stamp":  time.isoformat()}


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
