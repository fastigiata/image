import {ImageLoader} from "../../index.js"
import {readFileSync, writeFileSync} from "node:fs";

const load_trans_write = (ext: string) => {
    const buffer = readFileSync(`../_source/tree.${ext}`)
    const img = ImageLoader.fromPnm([...buffer])

    const png = img.toPng()
    writeFileSync(`../_out/${ext}.png`, Buffer.from(png))
}

load_trans_write('pbm')  // lose some data
load_trans_write('pgm')
load_trans_write('ppm')
load_trans_write('pam')
load_trans_write('pnm')