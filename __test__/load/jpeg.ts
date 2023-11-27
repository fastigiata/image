import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.jpeg")
const img = ImageLoader.fromJpeg([...buffer])

const png = img.toPng()
writeFileSync("../_out/jpeg.png", Buffer.from(png))