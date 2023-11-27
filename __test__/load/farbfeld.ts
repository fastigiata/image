import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.ff")
const img = ImageLoader.fromFarbfeld([...buffer])

const png = img.toPng()
writeFileSync("../_out/ff.png", Buffer.from(png))