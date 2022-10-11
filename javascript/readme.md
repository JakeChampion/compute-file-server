# c-at-e-file-server

JavaScript library used to serve files from Fastly Object Store.

It is ideal for serving files built from a static site generator such as 11ty.

## Install

### npm

```sh
npm install c-at-e-file-server
```

## API

### get(store_name, request)
Retrieve a file from a Fastly Object Store.

Returns: `response` or `null`

#### store_name
Type: `string`

The name of the Fastly Object Store to search within.

#### request
Type: `request`

The request to search for a corresponding response for.

> **Important:**
>   * If the request path ends in `/`, then `index.html` is appended to the path when searching the Object Store
>   * If the request path does not have an extension, then `index.html` is appended to the path when searching the Object Store


```js
import { get } from "c-at-e-file-server";

async function app(event) {
  const response = await get('site', request);
  if (response) {
    return response
  } else {
    return new Response(null, { status: 404 });
  }
}

addEventListener("fetch", event => event.respondWith(app(event)));
```
