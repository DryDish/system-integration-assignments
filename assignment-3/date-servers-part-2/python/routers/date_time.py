import datetime

from fastapi import APIRouter
from Datatypes.DateTimeResponse import DateTimeExample, DateTimeResponse
router = APIRouter()


# , responses={200: {"utc_time_stamp": "2022-09-11T21:55:26.253662725+00:00"}}

@router.get("/datetime", status_code=200, response_model=DateTimeExample)
def date_time():
    time = datetime.datetime.now(datetime.timezone.utc)
    return DateTimeResponse(time.isoformat())
