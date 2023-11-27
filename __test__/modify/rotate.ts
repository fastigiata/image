import {ImageLoader} from "../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("./tree.jpg")
const img = ImageLoader.fromJpeg([...buffer])

const img_rotated = img.rotateQuarter(1)

writeFileSync("./tree_rotated.jpg", Buffer.from(img_rotated.toJpeg(100)))