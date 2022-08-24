import * as fs from "fs";
import * as path from "path";

const fileToString = (filename: string) => {
	const filesFolder: string = "../../../../assignment-1/files/";
	const filePath: string = path.join(__dirname, filesFolder + filename);

	const file: string = fs.readFileSync(filePath, { encoding: "utf-8" });

	return file;
};

export { fileToString };
