import { lookup } from 'mrmime';
import parseRange from 'range-parser'

// TODO: Add support for validator fields -- https://httpwg.org/specs/rfc9110.html#response.validator
// last-modified https://httpwg.org/specs/rfc9110.html#field.last-modified
// etag https://httpwg.org/specs/rfc9110.html#field.etag

// TODO: Add support for conditional requests -- https://httpwg.org/specs/rfc9110.html#conditional.requests
// if-match https://httpwg.org/specs/rfc9110.html#field.if-match
// if-none-match https://httpwg.org/specs/rfc9110.html#field.if-none-match
// if-modified-since https://httpwg.org/specs/rfc9110.html#field.if-modified-since
// if-unmodified-since https://httpwg.org/specs/rfc9110.html#field.if-unmodified-since
// if-range https://httpwg.org/specs/rfc9110.html#field.if-range

/**
 * @param {string} store_name 
 * @param {Request} request 
 * @returns {Promise<Response | null>}
 */
export async function get(store_name, request) {
    const isHeadRequest = request.method === 'HEAD'
    // static files should only respond on HEAD and GET requests
    if (!isHeadRequest && request.method !== 'GET') {
        return null
    }

    // if path ends in / or does not have an extension
    // then append /index.html to the end so we can serve a page
    let path = new URL(request.url).pathname
    if (path.endsWith('/')) {
        path += 'index.html'
    } else if (!path.includes('.')) {
        path += '/index.html'
    }

    const item = await (new ObjectStore(store_name)).get(path)

    if (item == null) {
        return null
    }

    const headers = {
        'accept-ranges': 'bytes'
    }

    const extensionIndex = path.lastIndexOf('.');
    if (extensionIndex > -1) {
        const extension = path.substring(extensionIndex + 1)
        if (extension) {
            const mime = lookup(extension)
            if (mime) {
                headers['content-type'] = mime
            }
        }
    }

    const range = request.headers.get('range')
    if (range == null) {
        return new Response(isHeadRequest ? null : item.body, { status: 200, headers })
    } else {
        const itemBuffer = await item.arrayBuffer()
        const total = itemBuffer.byteLength
        const subranges = parseRange(total, range)

        // -1 signals an unsatisfiable range
        if (subranges == -1) {
            headers['content-range'] = `bytes */${total}`
            return new Response(null, { status: 416, headers })
        }
        // -2 signals a malformed header string
        if (subranges == -2) {
            headers['content-length'] = String(total)
            return new Response(isHeadRequest ? null : itemBuffer, { status: 200, headers })
        }

        if (subranges.length == 1) {
            const { start, end } = subranges[0]
            headers['content-range'] = `bytes ${start}-${end}/${total}`
            headers['content-length'] = String(end - start + 1)

            return new Response(isHeadRequest ? null : itemBuffer.slice(start, end), { status: 206, headers })
        } else {
            const mime = headers['content-type']
            headers['content-type'] = 'multipart/byteranges; boundary=3d6b6a416f9b5'
            const enc = new TextEncoder();
            const boundary = enc.encode(`--3d6b6a416f9b5\n`)
            const type = mime ? enc.encode(`Content-Type: ${mime}'\n`) : null
            const results = []
            subranges.forEach(function ({ start, end }) {
                results.push(boundary)
                type && results.push(type)
                results.push(enc.encode(`bytes ${start}-${end}/${total}`))
                results.push(itemBuffer.slice(start, end))
            })
            results.push(boundary)
            const body = concat(results)
            const length = body.byteLength
            headers['content-length'] = String(length)
            return new Response(isHeadRequest ? null : body, { status: 206, headers })
        }
    }
}

function concat(views) {
    let length = 0
    for (const v of views)
        length += v.byteLength

    let buf = new Uint8Array(length)
    let offset = 0
    for (const v of views) {
        const uint8view = new Uint8Array(v.buffer, v.byteOffset, v.byteLength)
        buf.set(uint8view, offset)
        offset += uint8view.byteLength
    }

    return buf
}