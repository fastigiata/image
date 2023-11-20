import {readFileSync} from "node:fs"
import {ImageLoader, sum} from "../index.js"

const raw = readFileSync('./tree.jpg')

const image = ImageLoader.autoGuess([...raw])
console.log(image.dimension())
