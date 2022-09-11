from logging import info, warn
import requests

from Datatypes.HttpResponse import HttpResponse, HttpResponseExample
from Datatypes.DateTimeResponse import DateTimeExample, DateTimeResponse


from fastapi import APIRouter

router = APIRouter()


@router.get("/datetime_from_rust",
            responses={500: {}, 200: {}},
            response_model=DateTimeExample
            )
def date_time_from_rust():
    try:
        response = requests.get("http://localhost:5000/datetime")
    except BaseException:
        warn(" --- Rust Server is unreachable.")
        return HttpResponse(500, "Other Server is unreachable.")

    if response.status_code == 200:
        info("SUCCESS")
        text = response.json()["utc_time_stamp"]
        return DateTimeResponse(text)
    elif response.status_code == 500:
        info("BAD")
        return HttpResponse.internal_server_error()
    else:
        info("HUH")
        return HttpResponse(418, "Unknown error.")
