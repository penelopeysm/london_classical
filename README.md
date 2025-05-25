# london-classical

Website which aggregates (some) classical music concerts in London.
View online at: https://penelopeysm.github.io/london_classical/

This is done by scraping the websites of various venues and aggregating the data into a single page.
The scraping is done with a small Rust application, located in the `rust` subdirectory.

The frontend is built with Svelte / TypeScript, and is in the `src` subdirectory.

## Which venue(s) does this code cover?

Scraping is currently implemented for classical music concerts at:

- ✅ Wigmore Hall
- ✅ BBC Proms 2025
- ✅ Southbank Centre

Things I probably would like to add, but haven't yet:

- ❌ Barbican
- ❌ Royal Albert Hall
- ❌ Cadogan Hall

Unfortunately, I haven't been able to get GitHub Actions runners to scrape Southbank concerts (due to Cloudflare).
I'm not sure if this can be fixed with headers, or if it's an IP block (which would probably necessitate using a proxy).
This means that the website, unfortunately, has to be built manually right now.

## Run locally

1. Clone the repository.

   ```
   git clone git@github.com:penelopeysm/london-classical.git
   cd london-classical
   ```

2. Run the Rust application to generate the list of concerts.

   ```
   pnpm rust:types
   pnpm rust
   ```

   You can set the following environment variables to control the Rust app:
    - `$LDNCLS_WIGMORE_MAX`: Maximum number of concerts to scrape from Wigmore Hall. Default: 220
    - `$LDNCLS_WIGMORE_DISABLE`: Any non-empty value will disable scraping Wigmore Hall concerts.
    - `$LDNCLS_PROMS_DISABLE`: Any non-empty value will disable scraping the Proms.

3. Run the website.

   ```
   pnpm install
   pnpm dev
   ```

## Generating types

The types associated with concerts are defined in `rust/src/core.rs`.
These types are exported to TypeScript using `ts-rs`, and live inside `src/lib/bindings`
To update the TypeScript bindings, run `pnpm rust:types`
