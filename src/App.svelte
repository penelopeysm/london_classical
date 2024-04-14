<script lang="ts">
    import Title from "src/lib/Title.svelte";
    import Filters from "src/lib/Filters.svelte";
    import Overview from "src/lib/Overview.svelte";
    import Details from "src/lib/Details.svelte";

    import { type Concert } from "src/types";
    let searchTerm: string = "";
    let selectedConcert: Concert | null = null;
</script>

<body>
    <main>
        <div class="top">
            <Title />
            <Filters bind:searchTerm />
        </div>
        <div class="bottom">
            <Overview bind:searchTerm bind:selectedConcert />
            <Details bind:selectedConcert />
        </div>
    </main>
</body>

<style>
    body {
        display: flex;
        justify-content: center;
        background-color: #fef0ff;
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
        display: flex;
        gap: 30px;
        width: 100%;
        min-height: 0;  /* So confusing that this is required */
        max-height: 100%;
    }
</style>
