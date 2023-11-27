import {ImageLoader} from "../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("./a.webp")
const img = ImageLoader.fromWebp([...buffer])

const png = img.toPng()

writeFileSync("./a.png", Buffer.from(png))