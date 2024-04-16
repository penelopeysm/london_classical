<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    import { type FiltersType } from "src/lib/filters";

    export let selectedConcert: Concert | null;
    export let filters: FiltersType;

    // Initialise list of concerts
    import rawConcerts from "src/assets/concerts.json";
    const concerts = rawConcerts.map((c) => {
        return {
            ...c,
            datetime: new Date(c.datetime),
        } as unknown as Concert;
    });
    let concertsToShow: Concert[] = [];

    function satisfiesFilters(concert: Concert, filters: FiltersType): boolean {
        // Check search filter
        let ciSearchTerm = filters.searchTerm.toLowerCase();
        let searchPass =
            filters.searchTerm === "" ||
            concert.title.toLowerCase().includes(ciSearchTerm) ||
            (concert.subtitle !== null &&
                concert.subtitle.toLowerCase().includes(ciSearchTerm)) ||
            concert.venue.toLowerCase().includes(ciSearchTerm) ||
            concert.performers.some((p) =>
                p.name.toLowerCase().includes(ciSearchTerm),
            );

        // Check U35 filter
        let u35Pass = filters.wigmoreU35 ? concert.is_wigmore_u35 : true;

        return searchPass && u35Pass;
    }

    $: {
        concertsToShow = concerts.filter((c) => satisfiesFilters(c, filters));
    }
</script>

<div class="overview">
    {#each concertsToShow as concert}
        <button
            class="concert"
            class:active={selectedConcert === concert}
            class:wigmoreU35={concert.is_wigmore_u35}
            on:click={() => {
                selectedConcert = concert;
            }}
        >
            <h3>{concert.title}</h3>
            {#if concert.subtitle !== null}
                <h4>{concert.subtitle}</h4>
            {/if}
            <p>{concert.datetime.toLocaleString()}</p>
            <p>{concert.venue}</p>
            <p>{concert.performers.join(", ")}</p>
        </button>
    {/each}
</div>

<style>
    .overview {
        display: flex;
        flex-direction: column;
        gap: 10px;
        width: 100%;
        height: 100%;
        overflow-y: auto;
    }

    .concert {
        border: 2px solid #000;
        padding: 10px;
        border-radius: 5px;
        width: 100%;

        font-family: inherit;
        text-align: left;
        cursor: pointer;
        transition: background-color 0.3s;
    }

    .active {
        background-color: #ff99bb;
    }

    .wigmoreU35 {
        border: 2px dashed #c13ad6;
    }

    .concert > * {
        margin: 0;
    }
</style>
