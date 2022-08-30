from http.client import OK
from libs import Json, Yaml, Xml, Csv, Txt, parse_path
from pprint import pprint
from fastapi import FastAPI
from dotenv import load_dotenv

import os
import uvicorn
import requests

load_dotenv(parse_path(".env"))

json_user = Json.parse("user.json")
yaml_user = Yaml.parse("user.yaml")
csv_user = Csv.parse("user.csv")
txt_user = Txt.parse("user.txt")
xml_user = Xml.parse("user.xml")

app = FastAPI()


@app.get("/")
def default():
    response = requests.get("http://localhost:3000/")
    print(response.content)
    return {"Hello": "World, Python"}


@app.get("/json")
def json_endpoint():
    return json_user


@app.get("/yaml")
def yaml_endpoint():
    return yaml_user


@app.get("/csv")
def csv_endpoint():
    return csv_user


@app.get("/txt")
def txt_endpoint():
    return txt_user


@app.get("/xml")
def xml_endpoint():
    return xml_user["root"]


@app.get("/print_test")
def print_test():
    print("___________JSON___________")
    pprint(json_user, width=80)
    print("___________YAML___________")
    pprint(yaml_user)
    print("___________TXT____________")
    pprint(txt_user)
    print("___________XML____________")
    pprint(xml_user["root"])
    print("___________CSV____________")
    pprint(csv_user)
    return OK


if __name__ == '__main__':
    uvicorn.run(
        "main:app",
        port=int(os.environ.get("PYTHON_PORT") or 1234),
        host='127.0.0.1',
        reload=True
    )
