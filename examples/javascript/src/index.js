/// <reference types="@fastly/js-compute" />

import { get } from "../../../libraries/javascript";

addEventListener("fetch", (event) => event.respondWith(app(event)));

async function app(event) {
  try {
    console.log(`FASTLY_SERVICE_VERSION: ${fastly.env.get('FASTLY_SERVICE_VERSION')}`)
    const response = await get('site', event.request)
    if (response) {
      return response
    } else {
      return new Response("Not Found", { status: 404 });
    }
  } catch (error) {
    console.error(error);
    return new Response(error.message + '\n' + error.stack, {status: 500})
  }
}
