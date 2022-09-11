from pydantic import BaseModel


class DateTimeResponse:
    utc_time_stamp: str

    def __init__(self, timestamp: str):
        self.utc_time_stamp = timestamp

    def __repr__(self) -> str:
        return f'"utc_time_stamp": {self.utc_time_stamp}'

    def __iter__(self):
        yield 'utc_time_stamp', self.utc_time_stamp


class DateTimeExample(BaseModel):
    utc_time_stamp: str = "2022-09-11T21:55:26.253662725+00:00"
