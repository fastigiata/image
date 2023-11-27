import {ImageLoader} from "../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("./tree.jpg")
const img = ImageLoader.fromJpeg([...buffer])

const img_resized = img.resizeToFit(200, 200)

writeFileSync("./tree_resized.jpg", Buffer.from(img_resized.toJpeg(100)))