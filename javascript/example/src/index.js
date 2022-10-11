/// <reference types="@fastly/js-compute" />

import { get } from "../../";

addEventListener("fetch", (event) => event.respondWith(app(event)));

async function app(event) {
  const response = get('site', event.request)
  if (response) {
    return response
  } else {
    return new Response("Not Found", { status: 404 });
  }
}
