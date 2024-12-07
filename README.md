# london-classical

Website which aggregates (some) classical music concerts in London.
View online at: https://penelopeysm.github.io/london_classical/

This is done by scraping the websites of various venues and aggregating the data into a single page.
The scraping is done with a small Rust application, located in the `rust` subdirectory.

The frontend is built with Svelte / TypeScript, and is in the `src` subdirectory.

## Which venue(s) does this code cover?

Scraping is currently implemented for concerts at:

- [x] Wigmore Hall
- [ ] Barbican
- [ ] Southbank Centre
- [x] BBC Proms
- [ ] Anything else at the Royal Albert Hall

Because the Proms aren't in season now, the website right now only lists Wigmore Hall concerts.

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

3. Run the website.

   ```
   pnpm install
   pnpm dev
   ```

## Generating types

The types associated with concerts are defined in `rust/src/core.rs`.
These types are exported to TypeScript using `ts-rs`, and live inside `src/lib/bindings`
To update the TypeScript bindings, run `pnpm rust:types`
