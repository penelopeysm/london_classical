<script lang="ts">
    import { type Concert } from "src/types";

    export let selectedConcert: Concert | null;
    export let searchTerm: string;

    // Initialise list of concerts
    import rawConcerts from "src/assets/concerts.json";
    const concerts = rawConcerts.map((c: any) => ({
        datetime: new Date(c.datetime),
        url: c.url,
        venue: "Wigmore Hall",
        title: c.title,
        performers: [c.performer],
        pieces: [],
    }));
    let concertsToShow: Concert[] = [];

    function satisfiesFilters(concert: Concert, searchTerm: string): boolean {
        if (searchTerm === "") {
            return true;
        }

        let ciSearchTerm = searchTerm.toLowerCase();
        return (
            concert.title.toLowerCase().includes(ciSearchTerm) ||
            concert.venue.toLowerCase().includes(ciSearchTerm) ||
            concert.performers.some((p) => p.toLowerCase().includes(ciSearchTerm))
        );
    }

    $: {
        concertsToShow = concerts.filter((c) => satisfiesFilters(c, searchTerm)); 
    }
</script>

<div class="overview">
    {#each concertsToShow as concert}
        <button
            class="concert"
            class:active={selectedConcert === concert}
            on:click={() => {selectedConcert = concert}}
        >
            <h3>{concert.title}</h3>
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
        width: 50%;
        max-width: 50%;
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

    .concert > * {
        margin: 0;
    }
</style>
