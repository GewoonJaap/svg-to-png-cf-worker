![afbeelding](https://user-images.githubusercontent.com/33700526/207815865-9b471652-5723-4d35-8847-dce0fb9701eb.png)

# SVG to PNG Cloudflare Worker

SVG to PNG converter in Cloudflare Workers

# Installation

## Windows Specific
- Install [Strawberry Perl](https://strawberryperl.com/)

## All OS
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Cloudflare Wrangler](https://developers.cloudflare.com/workers/cli-wrangler/install-update)
- `wrangler login`
- Create a Cloudflare worker with name: `svg-to-png`;
- `wrangler dev` to local test
- `wrangler publish` to publish to Cloudflare

# Usage

`https://svg-to-png.mrproper.dev/{SVG URL}`

**Demo**: https://svg-to-png.mrproper.dev/https://docs.tandoor.dev/logo_color.svg
