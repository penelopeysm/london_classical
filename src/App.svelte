<script lang="ts">
    import Title from "src/components/Title.svelte";
    import Filters from "src/components/Filters.svelte";
    import Overview from "src/components/Overview.svelte";
    import Details from "src/components/Details.svelte";
</script>

<body>
    <main>
        <div class="top">
            <Title />
        </div>
        <div class="bottom">
            <Overview />
            <div class="bottom-right">
                <Filters />
                <Details />
            </div>
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
        gap: 0px;
        align-items: center;

        width: 80%;
        min-height: 100%;
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
        grid-template-columns: 40% 60%;
        gap: 20px;
        width: 100%;
        max-width: 100%;
        min-height: 0; /* So confusing that this is required */
        height: 100%;
        max-height: 100%;
    }

    div.bottom-right {
        display: flex;
        flex-direction: column;
        gap: 20px;
        width: 100%;
        height: 100%;
        min-height: 0;
        max-height: 100%;
    }
</style>
