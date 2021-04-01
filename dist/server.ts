async function handleRequest(request: Request) {
  const { pathname } = new URL(request.url);

  if (pathname === "/") {
    return new Response(
      `<!DOCTYPE html><html><head>
      <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.2/css/bulma.min.css">
      <link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.6.1/css/all.css">
      <link rel="stylesheet" href="index-885242d574d12a36.css">
      <meta charset="utf-8">
      <meta name="viewport" content="width=device-width,initial-scale=1">
      <title>Hironori Yamamoto Portfolio</title>
      <script type="module">import init from 'index-279a11c6247de836.js';init('index-279a11c6247de836_bg.wasm');</script></head>
      <body>
      </body></html>`,
      {
        headers: {
          "content-type": "text/html; charset=utf-8",
        },
      },
    );
  }
  if (pathname.match(/^index-\w+\.(wasm|js|css)$/)) {
    const content = new URL(`${pathname}`, import.meta.url);
    return fetch(content);
  }

  return new Response(
    JSON.stringify({ message: "not found" }),
    {
      status: 404,
      headers: {
        "content-type": "application/json; charset=UTF-8",
      },
    },
  );
}

addEventListener("fetch", (event: FetchEvent) => {
  event.respondWith(handleRequest(event.request));
});