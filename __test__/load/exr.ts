import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.exr")
const img = ImageLoader.fromOpenEXR([...buffer])

const png = img.toPng()
writeFileSync("../_out/exr.png", Buffer.from(png))