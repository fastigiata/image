import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.jpeg")
const img = ImageLoader.fromJpeg([...buffer])

const img_cropped = img.crop(0, 0, 250, 90)

writeFileSync("../_out/crop.jpeg", Buffer.from(img_cropped.toJpeg(100)))