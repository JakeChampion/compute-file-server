
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'
const __dirname = dirname(fileURLToPath(import.meta.url))
let location = join(__dirname, 'c-at-e-file-server')
export default location
