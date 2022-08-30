import json
from posixpath import dirname
import yaml
import xml
import xmltodict
import csv


def parse_path(path):
    current_path = dirname(__file__)
    return current_path + "/../../files/" + path


class Json():
    def parse(io_file: json) -> str:
        io_file = parse_path(io_file)
        file = open(io_file, "rt")
        parsed_user = json.loads(file.read())
        file.close()

        return parsed_user


class Yaml:
    def parse(io_file: yaml) -> dict:
        io_file = parse_path(io_file)
        file = open(io_file, "rt")
        parsed_user = yaml.safe_load(file.read())
        file.close()

        return parsed_user


class Xml:
    def parse(io_file: xml) -> dict:
        io_file = parse_path(io_file)
        file = open(io_file, "r")
        parsed_user = xmltodict.parse(file.read())
        return parsed_user


class Csv:
    def parse(io_file: csv) -> dict:
        io_file = parse_path(io_file)
        file = open(io_file, "r")
        reader = csv.DictReader(file)

        return reader.__next__()


class Txt:
    def __to_dict(str1, str2) -> dict:
        return {str1.strip(): str2.replace('\n', '')}

    def parse(io_file) -> dict:
        io_file = parse_path(io_file)
        file = open(io_file, "rt")
        nested = 0
        nested_dict_name = ""
        nested_dict = {}
        root_dict = {}
        for line in file:
            if line.startswith('#'):
                continue
            lines = line.split(",")
            if (len(lines) == 2):
                if lines[1] != '\n':
                    if (nested):
                        nested_dict.update(Txt.__to_dict(lines[0], lines[1]))
                    else:
                        root_dict.update(Txt.__to_dict(lines[0], lines[1]))
                else:
                    nested = 1
                    nested_dict_name = lines[0]
            elif len(lines) < 2:
                root_dict.update({nested_dict_name: nested_dict})
                nested = 0
                nested_dict = {}
            else:
                list = [word.strip() for word in line.split('"')[1].split(",")]
                root_dict.update({lines[0]: list})

        return root_dict
