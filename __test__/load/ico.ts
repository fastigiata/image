import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.ico")
const img = ImageLoader.fromIco([...buffer])

const png = img.toPng()
writeFileSync("../_out/ico.png", Buffer.from(png))