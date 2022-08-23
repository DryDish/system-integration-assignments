import json
import yaml
import xml
from xml.dom import minidom

class Json():
    def str_parse(file: json) -> str:
        open_file = open(file, "rt")
        parsed_user = json.loads(open_file.read())
        open_file.close()

        return parsed_user

class Yaml:
    def str_parse(file: yaml) -> str:
        open_file = open(file, "rt")
        parsed_user = yaml.safe_load(open_file.read())
        open_file.close();

        return parsed_user;

class Xml:
    def str_parse(file: xml) -> str:
        open_file = open(file, "rt")
        parsed_user = minidom.parse(open_file).toxml()

        return parsed_user