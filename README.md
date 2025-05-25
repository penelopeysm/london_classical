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

Things I really would like to add, but haven't yet:

- ❌ Barbican

I might get around to these at some point, but they're lower priority:

- ❌ Royal Albert Hall
- ❌ Cadogan Hall
- ❌ St Martin-in-the-Fields

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

## Deployment

Unfortunately, I haven't been able to get GitHub Actions runners to scrape Southbank concerts (due to Cloudflare).
I think this is because of an IP block.
This means that the scraping has to be done locally, and the results uploaded to GitHub (the website itself is still built using GitHub Actions) using:

```
pnpm rust
pnpm gh
```

The first step generates the JSON file with the concert data.
The second step uploads this file to [a GitHub release](https://github.com/penelopeysm/london_classical/releases/tag/json), and triggers the deployment GiHub Action (which fetches the JSON file and builds the website).

Note that you will need to be logged into GitHub CLI for this to work.
The user you are logged in as must also have the necessary permissions (i.e., you must be me).
