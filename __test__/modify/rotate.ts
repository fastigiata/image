import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.jpeg")
const img = ImageLoader.fromJpeg([...buffer])

const img_rotated = img.rotateQuarter(1)

writeFileSync("../_out/rotate.jpeg", Buffer.from(img_rotated.toJpeg(100)))