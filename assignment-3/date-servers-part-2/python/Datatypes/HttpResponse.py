from pydantic import BaseModel


class HttpResponse():
    status: int
    message: str

    def __init__(self, status, message):
        self.status = status
        self.message = message

    def __repr__(self) -> str:
        return f'"status": {self.status}, "message": {self.message}'

    def __iter__(self):
        yield 'status', self.status
        yield 'message', self.message

    def internal_server_error(self):
        self.status = 500
        self.message = "Internal Server Error"


class HttpResponseExample(BaseModel):
    status: int = 500
    message: str = "Internal Server Error"
