import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.tiff")
const img = ImageLoader.fromTiff([...buffer])

const png = img.toPng()
writeFileSync("../_out/tiff.png", Buffer.from(png))