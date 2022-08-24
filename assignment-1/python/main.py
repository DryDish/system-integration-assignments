from libs import Json, Yaml, Xml, Csv, Txt


json_user_string = Json.str_parse("user.json");
yaml_user_string = Yaml.str_parse("user.yaml");
xml_user_string = Xml.str_parse("user.xml");
csv_user_string = Csv.str_parse("user.csv");
txt_user_string = Txt.str_parse("user.txt")

print("___________JSON___________")
print(json_user_string)
print("___________YAML___________")
print(yaml_user_string)
print("___________TXT____________")
print(txt_user_string)
print("___________CSV____________")
print(csv_user_string)
print("___________XML____________")
print(xml_user_string)
