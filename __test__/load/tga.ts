import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.tga")
const img = ImageLoader.fromTga([...buffer])

const png = img.toPng()
writeFileSync("../_out/tga.png", Buffer.from(png))