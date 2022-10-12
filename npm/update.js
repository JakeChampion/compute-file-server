#!/usr/bin/env node

import { fileURLToPath } from 'node:url';
import { dirname, join, parse } from 'node:path';
import { fetch } from 'zx'
import { mkdir, writeFile } from "node:fs/promises";
import decompress from 'decompress';
import decompressUnzip from 'decompress-unzip';
import decompressTarxz from '@felipecrs/decompress-tarxz';

const __dirname = dirname(fileURLToPath(import.meta.url));
const tag = 'dev';
let response = await fetch(`https://api.github.com/repos/JakeChampion/c-at-e-file-server/releases/tags/${tag}`)
response = await response.json()
const id = response.id
let packages = {
    'c-at-e-file-server-cli-darwin-arm64': {
        releaseAsset: `c-at-e-file-server-${tag}-x86_64-macos.tar.xz`,
        binaryAsset: 'c-at-e-file-server',
        description: 'The macOS 64-bit binary for c-at-e-file-server',
        os: 'darwin',
        cpu: 'arm64',
    },
    'c-at-e-file-server-cli-darwin-x64': {
        releaseAsset: `c-at-e-file-server-${tag}-x86_64-macos.tar.xz`,
        binaryAsset: 'c-at-e-file-server',
        description: 'The macOS 64-bit binary for c-at-e-file-server',
        os: 'darwin',
        cpu: 'x64',
    },
    'c-at-e-file-server-cli-linux-x64': {
        releaseAsset: `c-at-e-file-server-${tag}-x86_64-linux.tar.xz`,
        binaryAsset: 'c-at-e-file-server',
        description: 'The Linux 64-bit binary for c-at-e-file-server',
        os: 'darwin',
        cpu: 'x64',
    },
    'c-at-e-file-server-cli-linux-s390x': {
        releaseAsset: `c-at-e-file-server-${tag}-x86_64-linux.tar.xz`,
        binaryAsset: 'c-at-e-file-server',
        description: 'The Linux s390x binary for c-at-e-file-server',
        os: 'darwin',
        cpu: 's390x',
    },
    'c-at-e-file-server-cli-win32-x64': {
        releaseAsset: `c-at-e-file-server-${tag}-x86_64-windows.zip`,
        binaryAsset: 'c-at-e-file-server.exe',
        description: 'The Windows 64-bit binary for c-at-e-file-server',
        os: 'win32',
        cpu: 'x64',
    },
}
let assets = await fetch(`https://api.github.com/repos/JakeChampion/c-at-e-file-server/releases/${id}/assets`)
assets = await assets.json()
console.log(assets)
for (const [packageName, info] of Object.entries(packages)) {
    const asset = assets.find(asset => asset.name === info.releaseAsset)
    if (!asset) {
        throw new Error(`Can't find an asset named ${info.releaseAsset} for the release https://github.com/JakeChampion/c-at-e-file-server/releases/tag/${tag}`)
    }
    const packageDirectory = join(__dirname, '../', packageName.split('/').pop())
    await mkdir(packageDirectory, { recursive: true })
    await writeFile(join(packageDirectory, 'package.json'), packageJson(packageName, tag, info.description, info.os, info.cpu))
    await writeFile(join(packageDirectory, 'index.js'), indexJs(info.binaryAsset))
    const browser_download_url = asset.browser_download_url;
    let archive = await fetch(browser_download_url)
    await decompress(Buffer.from(await archive.arrayBuffer()), packageDirectory, {
        strip:1,
        plugins: [
            decompressTarxz(),
            decompressUnzip()
        ],
        filter: file => parse(file.path).base === info.binaryAsset
    })
}

function indexJs(binaryAsset) {
    return `
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'
const __dirname = dirname(fileURLToPath(import.meta.url))
let location = join(__dirname, '${binaryAsset}')
export default location
`
}
function packageJson(name, version, description, os, cpu) {
    return JSON.stringify({
        name: `@fastly/${name}`,
        bin: {
            [name]: "c-at-e-file-server"
        },
        type: "module",
        version,
        main: "index.js",
        description,
        license: "Apache-2.0",
        preferUnplugged: false,
        os: [os],
        cpu: [cpu],
    }, null, 4);
}
