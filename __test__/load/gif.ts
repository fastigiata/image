import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.gif")
const img = ImageLoader.fromGif([...buffer])

const png = img.toPng()
writeFileSync("../_out/gif.png", Buffer.from(png))