import json
from typing import Literal
import yaml
import xml
from xml.dom import minidom
import csv

class Json():
    def str_parse(io_file: json) -> str:
        file = open(io_file, "rt")
        parsed_user = json.loads(file.read())
        file.close()

        return parsed_user

class Yaml:
    def str_parse(io_file: yaml) -> str:
        file = open(io_file, "rt")
        parsed_user = yaml.safe_load(file.read())
        file.close();

        return parsed_user;

class Xml:
    def str_parse(io_file: xml) -> str:
        file = open(io_file, "rt")
        parsed_user = minidom.parse(file).toxml()

        return parsed_user


class Csv:
    def str_parse(io_file: csv) -> str:
        file = open(io_file, "r")
        reader = csv.DictReader(file)

        return reader.__next__();

class Txt:
    def to_dict(str1, str2) -> dict:
        return {str1.strip() : str2.replace('\n', '')}

    def str_parse(io_file) -> str:
        file = open(io_file, "rt")

        nested = 0
        nest_name = ""
        nested_dict = {}
        something = {}
        for line in file:
            if line.startswith('#'):
                continue
            lines = line.split(",")
            if (len(lines) == 2 ):
                if lines[1] != '\n':
                    if (nested):
                        nested_dict.update(Txt.to_dict(lines[0], lines[1]))
                    else:
                        something.update(Txt.to_dict(lines[0], lines[1]))
                else:
                    nested = 1
                    nest_name = lines[0]
            elif len(lines) < 2:
                print('smol line')
                something.update({nest_name: nested_dict})
                nested = 0
                nested_dict = {}
            else:
                list = line.split('"')[1]
                something.update({lines[0]: [list]})

        return something

