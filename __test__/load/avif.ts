import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.avif")
const img = ImageLoader.fromAvif([...buffer])

const png = img.toPng()
writeFileSync("../_out/avif.png", Buffer.from(png))