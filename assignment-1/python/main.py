from libs import Json, Yaml, Xml, Csv, Txt
from pprint import pprint

json_user_string = Json.parse("user.json");
yaml_user_string = Yaml.parse("user.yaml");
csv_user_string = Csv.parse("user.csv");
txt_user_string = Txt.parse("user.txt")
xml_user_string = Xml.parse("user.xml");

print("___________JSON___________")
pprint(json_user_string, width=80)
print("___________YAML___________")
pprint(yaml_user_string)
print("___________TXT____________")
pprint(txt_user_string)
print("___________XML____________")
pprint(xml_user_string["root"])
print("___________CSV____________")
pprint(csv_user_string)

