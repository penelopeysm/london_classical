<script lang="ts">
    import Title from "src/lib/Title.svelte";
    import Filters from "src/lib/Filters.svelte";
    import Overview from "src/lib/Overview.svelte";
    import Details from "src/lib/Details.svelte";

    import { type FiltersType } from "src/lib/filters";
    import { type Concert } from "src/lib/bindings/Concert";
    let selectedConcert: Concert | null = null;

    let filters: Filters = {
        searchTerm: "",
        wigmoreU35: false,
    };

    let searchTerm: string = "";
</script>

<body>
    <main>
        <div class="top">
            <Title />
            <Filters bind:filters />
        </div>
        <div class="bottom">
            <Overview bind:filters bind:selectedConcert />
            <Details bind:selectedConcert />
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
        min-height: 0;  /* So confusing that this is required */
        max-height: 100%;
    }
</style>
