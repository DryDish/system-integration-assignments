import { parse } from "yaml";
import * as Papa from "papaparse";

const parseJson = (file: string) => {
	return JSON.parse(file);
};

const parseYaml = (file: string) => {
	const parsedYaml: string = parse(file);
	return parsedYaml;
};

const parseTxt = (file: string) => {
	// Split string into  lines on newline char
	const lines = file.split(/\r?\n/);

	let nested = false;
	let nestedObjName = "";
	let nestedObj: any = {};
	let rootObj: any = {};

	lines.forEach((line: string) => {
		const lineArray = splitString(line, ",");
		if (lineArray?.length == 2) {
			if (lineArray[1] != "") {
				if (nested) {
					nestedObj[lineArray[0].trim()] = lineArray[1].trim();
				} else {
					rootObj[lineArray[0]] = lineArray[1].trim();
				}
			} else {
				nested = true;
				nestedObjName = lineArray[0];
			}
		} else if (lineArray?.length == 1) {
			rootObj[nestedObjName] = nestedObj;
		} else if (lineArray != null) {
			const list = line
				.split('"')[1]
				.split(",")
				.map((word) => {
					return word.trim();
				});
			rootObj[lineArray[0].trim()] = list;
		}
	});

	return rootObj;
};

const parseCsv = (file: string) => {
	const rootObj: any = {};
	const options = {
		header: true,
		transformHeader: (header: string) => {
			if (header.includes(".")) {
				header = header.replace(".", "_");
			}
			return header;
		},
	};
	const parsedCsv = Papa.parse(file, options);

	return parsedCsv.data[0];
};

import * as xml2js from "xml2js";
const parseXml = (file: string) => {
	let result: any = "result";
	xml2js.parseString(file, (err: any, res: any) => {
		if (err) {
			console.log(err);
		}
		result = res;
	});
	return result.root;
};

const splitString = (string: string, delimiter: string) => {
	if (string.startsWith("#")) {
		return null;
	} else {
		return string.split(delimiter).map((word) => word.trim());
	}
};

export { parseJson, parseYaml, parseTxt, parseCsv, parseXml };
