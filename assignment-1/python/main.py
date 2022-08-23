from libs import Json
from libs import Yaml
from libs import Xml

json_user_string = Json.str_parse("assignment-1/files/user.json");
yaml_user_string = Yaml.str_parse("assignment-1/files/user.yaml");
xml_user_string = Xml.str_parse("assignment-1/files/user.xml");


print("___________JSON___________")
print(json_user_string)
print("___________YAML___________")
print(yaml_user_string)
print("___________XML____________")
print(xml_user_string)
print("___________CSV____________")
print("TODO!")
print("___________TXT____________")
print("TODO!")