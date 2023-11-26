import {ImageLoader} from "../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("./treex2.jpg")
const img = ImageLoader.fromJpeg([...buffer])

const ico = img.toIco('exact_nearest')

writeFileSync("./treex2.ico", Buffer.from(ico))