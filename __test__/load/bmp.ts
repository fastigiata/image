import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.bmp")
const img = ImageLoader.fromBmp([...buffer])

const png = img.toPng()
writeFileSync("../_out/bmp.png", Buffer.from(png))