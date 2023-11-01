/// <reference types="@fastly/js-compute" />

import { get } from "compute-file-server";

addEventListener("fetch", (event) => event.respondWith(app(event)));

async function app(event) {
  try {
    console.log(`FASTLY_SERVICE_VERSION: ${fastly.env.get('FASTLY_SERVICE_VERSION')}`)
    const response = await get('site', event.request)
    if (response) {
      // Enable Dynamic Compression -- https://developer.fastly.com/learning/concepts/compression/#dynamic-compression
      response.headers.set("x-compress-hint", "on");
      return response
    } else {
      return new Response("Not Found", { status: 404 });
    }
  } catch (error) {
    console.error(error);
    return new Response(error.message + '\n' + error.stack, {status: 500})
  }
}
