import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.qoi")
const img = ImageLoader.fromQoi([...buffer])

const png = img.toPng()
writeFileSync("../_out/qoi.png", Buffer.from(png))