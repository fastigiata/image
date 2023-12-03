import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.jpeg")
const img = ImageLoader.fromJpeg([...buffer])

const img_flipped = img.flip(false)

writeFileSync("../_out/flip.jpeg", Buffer.from(img_flipped.toJpeg(100)))