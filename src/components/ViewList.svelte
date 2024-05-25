<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    import {
        concertViews,
        filters,
        currentViewName,
        selectedConcertIndices,
    } from "src/lib/stores";
    import { initialFilters } from "src/lib/filters";

    export let allConcerts: Concert[];
    export let shownIndices: number[];

    function setViewName(newViewName: string) {
        $currentViewName = newViewName;
    }

    function addEmptyView() {
        const newViewName = getNewViewName();
        if (newViewName === null) {
            return;
        }
        $concertViews.set(newViewName, []);
        $concertViews = new Map($concertViews); // Required to trigger store update
        $currentViewName = newViewName;
    }

    function addViewFromShownConcerts() {
        const newViewName = getNewViewName();
        if (newViewName === null) {
            return;
        }
        const shownConcerts = shownIndices.map((i) => allConcerts[i]);
        $filters = initialFilters;
        $concertViews.set(newViewName, shownConcerts);
        $concertViews = new Map($concertViews); // Required to trigger store update
        $currentViewName = newViewName;
    }

    function addViewFromSelectedConcerts() {
        const newViewName = getNewViewName();
        if (newViewName === null) {
            return;
        }
        const selectedConcerts = $selectedConcertIndices.map(
            (i) => allConcerts[i],
        );
        $filters = initialFilters;
        $concertViews.set(newViewName, selectedConcerts);
        $concertViews = new Map($concertViews); // Required to trigger store update
        $currentViewName = newViewName;
    }

    function getNewViewName(): string | null {
        const newViewName = prompt("Enter a name for the new view");
        if (newViewName === null) {
            return null;
        }
        if (newViewName === "") {
            alert("Please enter a name");
            return null;
        }
        if ($concertViews.has(newViewName)) {
            alert("A view with that name already exists");
            return null;
        }
        return newViewName;
    }

    function deleteView(viewName: string) {
        $concertViews.delete(viewName);
        $concertViews = new Map($concertViews); // Required to trigger store update
        $currentViewName = "All";
    }

    function exportView(viewName: string) {
        let concertIds = notUndefined($concertViews.get(viewName)).map(
            (c) => c.id,
        );
        console.log(JSON.stringify({ viewName, concerts: concertIds }));
    }

    // Silence type errors
    function notUndefined<T>(value: T | undefined): T {
        if (value === undefined) {
            throw new Error("Value is undefined");
        }
        return value;
    }
</script>

<div class="view-list">
    {#each $concertViews.keys() as viewName}
        {@const allConcertsLength = notUndefined(
            $concertViews.get(viewName),
        ).length}
        {@const shownConcertsLength = shownIndices.length}
        {#if viewName === "All"}
            <button
                class="view-button"
                class:active={$currentViewName === viewName}
                on:click={() => setViewName(viewName)}
            >
                {#if $currentViewName === viewName && shownConcertsLength !== allConcertsLength}
                    {viewName} ({shownConcertsLength}/{allConcertsLength})
                {:else}
                    {viewName} ({allConcertsLength})
                {/if}
            </button>
        {:else}
            <div class="dropdown-trigger">
                <button
                    class="view-button add-new-view"
                    class:active={$currentViewName === viewName}
                    on:click={() => setViewName(viewName)}
                >
                    {#if $currentViewName === viewName && shownConcertsLength !== allConcertsLength}
                        {viewName} ({shownConcertsLength}/{allConcertsLength})
                    {:else}
                        {viewName} ({allConcertsLength})
                    {/if}
                    <span class="smol">▼</span>
                    <div class="dropdown-options">
                        <button on:click={() => exportView(viewName)}
                            >Export view to JSON</button
                        >
                        <button on:click={() => deleteView(viewName)}
                            >Delete view</button
                        >
                    </div>
                </button>
            </div>
        {/if}
    {/each}
    <div class="dropdown-trigger">
        <button class="view-button add-new-view">
            Add new view <span class="smol">▼</span>
            <div class="dropdown-options">
                <button on:click={addEmptyView}>New empty view</button>
                <button on:click={addViewFromShownConcerts}
                    >... from currently shown concerts</button
                >
                <button on:click={addViewFromSelectedConcerts}
                    >... from currently selected concerts</button
                >
            </div>
        </button>
    </div>
</div>

<style>
    button.view-button {
        background-color: #f0f0f0;
        border: 1px solid #ccc;
        border-radius: 5px;
        padding: 5px;

        transition: all 0.3s;
        transition-property: background-color, border-color, box-shadow;
    }

    button.active {
        background-color: #c1eaf5;
        border-color: #32aecf;
        box-shadow: 0 0 3px #32aecf;
    }

    div.view-list {
        display: flex;
        flex-wrap: wrap;
        gap: 7px;
        align-items: baseline;
    }

    button.add-new-view {
        position: relative;
    }

    div.dropdown-options {
        display: none;
    }

    button.add-new-view:hover > div.dropdown-options {
        display: flex;
        flex-direction: column;
        gap: 0px;
        position: absolute;
        top: 27px;
        left: -1px;
        width: max-content;
        z-index: 1;
    }

    div.dropdown-options button {
        background-color: #f0f0f0;
        border: 1px solid #ccc;
        border-radius: 5px;
        padding: 5px;
        margin: 0;
        width: 100%;
        text-align: left;
    }

    div.dropdown-options button:hover {
        background-color: #ddd;
    }

    span.smol {
        font-size: 0.7em;
        margin-left: 2px;
    }
</style>
