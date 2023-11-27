import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../source/tree.webp")
const img = ImageLoader.fromWebp([...buffer])

const png = img.toPng()

writeFileSync("../out/webp.png", Buffer.from(png))