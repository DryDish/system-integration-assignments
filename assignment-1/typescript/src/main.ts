import { fileToString } from "./utils/file-reader";
import {
	parseCsv,
	parseJson,
	parseTxt,
	parseYaml,
	parseXml,
} from "./utils/parsers";


const userJson = parseJson(fileToString("user.json"));
const userYaml = parseYaml(fileToString("user.yaml"));
const userTxt =  parseTxt(fileToString("user.txt"))
const userCsv =  parseCsv(fileToString("user.csv"));
const userXml =  parseXml(fileToString("user.xml"));

console.log("___________JSON___________");
console.log(userJson);
console.log("___________YAML___________");
console.log(userYaml);
console.log("___________TXT___________");
console.log(userTxt);
console.log("___________CSV___________");
console.log(userCsv);
console.log("___________XML___________");
console.log("%o", userXml)
