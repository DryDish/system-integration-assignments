from fastapi import FastAPI

import uvicorn
from routers import date_time_router, date_time_from_rust_router


app = FastAPI(title="Python Date Server")

app.include_router(date_time_router)
app.include_router(date_time_from_rust_router)

if __name__ == '__main__':
    port = 3000
    host = '127.0.0.1'
    uvicorn.run(
        "main:app",
        port=port,
        host=host,
        reload=True,
    )
