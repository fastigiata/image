import {ImageLoader} from "../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("./tree.jpg")
const img = ImageLoader.fromJpeg([...buffer])

const img_cropped = img.crop(0, 0, 250, 90)

writeFileSync("./tree_cropped.jpg", Buffer.from(img_cropped.toJpeg(100)))