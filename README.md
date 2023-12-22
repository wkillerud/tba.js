# tba-js

To Be Announced

## Getting Started

```
npx tba-js compile
```

Or install with `npm i -D tba-js` and add this script to `package.json`:

```json
{
  "scripts: {
    "build": "tba compile"
  }
}
```

## Contributing

```
npm install
# edit src/lib.rs
npm run build
```

See [napi-rs](https://napi.rs/docs/introduction/simple-package) and [this blog post](https://www.lekoarts.de/garden/publishing-a-rust-cli-on-npm/) for more.

Publish via GitHub Actions (configure NPM_TOKEN as a secret).
