import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.jpeg")
const img = ImageLoader.fromJpeg([...buffer])

const img_resized = img.resizeToFit(200, 200)

writeFileSync("../_out/resize.jpeg", Buffer.from(img_resized.toJpeg(100)))