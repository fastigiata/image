import {ImageLoader} from "../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("./tree.jpg")
const img = ImageLoader.fromJpeg([...buffer])

const img_flipped = img.flip(false)

writeFileSync("./tree_flipped.jpg", Buffer.from(img_flipped.toJpeg(100)))