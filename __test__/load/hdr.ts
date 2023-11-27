import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const buffer = readFileSync("../_source/tree.hdr")
const img = ImageLoader.fromHdr([...buffer])

const png = img.toPng()
writeFileSync("../_out/hdr.png", Buffer.from(png))