import {ImageLoader} from "../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("./tree.jpg")
const img = ImageLoader.fromJpeg([...buffer])

const img2 = img.resizeToFit(200, 200)

writeFileSync("./tree2.jpg", Buffer.from(img2.toJpeg(100)))