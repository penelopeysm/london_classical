<script lang="ts">
    import Title from "src/lib/Title.svelte";
    import Filters from "src/lib/Filters.svelte";
    import Overview from "src/lib/Overview.svelte";
    import Details from "src/lib/Details.svelte";
    import { type Concert } from "src/lib/bindings/Concert";
    import { type HashedConcert, hashConcert } from "src/utils";
    import { type FiltersType, satisfies } from "src/lib/filters";

    // Initialise list of concerts
    import rawConcerts from "src/assets/concerts.json";
    const concerts: HashedConcert[] = rawConcerts.map(hashConcert);

    // Filter concerts
    let filters: FiltersType = {
        searchTerm: "",
        booleanTagNames: [],
    };
    let concertsToShow: HashedConcert[];
    let selectedConcertHashes: string[] = [];
    let selectedConcerts: HashedConcert[];
    $: {
        concertsToShow = concerts.filter((c) => satisfies(c, filters));
        selectedConcerts = concertsToShow.filter((c) =>
            selectedConcertHashes.includes(c.hash),
        );
    }
</script>

<body>
    <main>
        <div class="top">
            <Title />
            <Filters bind:filters />
        </div>
        <div class="bottom">
            <Overview concerts={concertsToShow} bind:selectedConcertHashes />
            <Details bind:selectedConcerts />
        </div>
    </main>
</body>

<style>
    body {
        display: flex;
        justify-content: center;
        background-color: #f5f5f5;
        height: 100vh;
        overflow: hidden;
    }

    main {
        display: flex;
        flex-direction: column;
        gap: 20px;
        align-items: center;

        width: 80%;
        height: 100%;
        max-height: 100%;
        padding-top: 30px;
        padding-bottom: 30px;
    }

    div.top {
        width: 100%;
        height: max-content;
        max-height: 100%;
    }

    div.bottom {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 30px;
        width: 100%;
        min-height: 0; /* So confusing that this is required */
        max-height: 100%;
    }
</style>
