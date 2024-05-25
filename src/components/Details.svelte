<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    import SelectedConcertDetails from "src/components/SelectedConcertDetails.svelte";
    import {
        concertViews,
        currentViewName,
        selectedConcertIndices,
        defaultViewName,
    } from "src/lib/stores";
    import Dropdown from "src/components/Dropdown.svelte";

    let allConcerts: Concert[] = $concertViews.get(
        $currentViewName,
    ) as Concert[];
    let selectedConcerts: Concert[] = allConcerts.filter((_, idx) =>
        $selectedConcertIndices.includes(idx),
    );

    function compareConcertTime(a: Concert, b: Concert): number {
        let aDate = new Date(a.datetime);
        let bDate = new Date(b.datetime);
        return aDate.getTime() - bDate.getTime();
    }

    // TODO Reduce duplication with ViewList.svelte
    function addToView(concerts: Concert[], viewName: string) {
        const existingConcerts = $concertViews.get(viewName) as Concert[];
        // Remove dupes and sort by date
        const mergedConcerts = [...existingConcerts, ...concerts]
            .filter((c, idx, arr) => arr.indexOf(c) === idx)
            .sort(compareConcertTime);
        $concertViews.set(viewName, mergedConcerts);
        $concertViews = new Map($concertViews); // trigger store update
    }

    function addToNewView(concerts: Concert[]) {
        const newViewName = getNewViewName();
        if (newViewName === null) {
            return;
        }
        $concertViews.set(newViewName, concerts);
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

    $: {
        allConcerts = $concertViews.get($currentViewName) as Concert[];
        selectedConcerts = allConcerts.filter((_, idx) =>
            $selectedConcertIndices.includes(idx),
        );
    }
</script>

<div class="details">
    {#if selectedConcerts.length === 1}
        <SelectedConcertDetails selectedConcert={selectedConcerts[0]} />
    {:else if selectedConcerts.length === 0}
        <div id="centred-text">
            <h2>No concert selected</h2>
            <p>Select a concert from the list on the left to view details :)</p>
            <p class="italic">
                (Tip: Use shift-click to select multiple concerts)
            </p>
        </div>
    {:else}
        <div id="centred-text">
            <h2>{selectedConcerts.length} concerts selected</h2>
            <div id="selected-concerts-summary">
                {#each selectedConcerts as concert}
                    <p>
                        <a href={concert.url} target="_blank">
                            {concert.title}
                        </a>
                    </p>
                {/each}
            </div>

            <Dropdown>
                <span slot="text">Add to view</span>
                <svelte:fragment slot="options">
                    {#each $concertViews
                        .keys()
                        .filter((k) => k !== defaultViewName) as view}
                        <button
                            on:click={() => {
                                addToView(selectedConcerts, view);
                            }}
                        >
                            {view}
                        </button>
                    {/each}
                    <button
                        on:click={() => {
                            addToNewView(selectedConcerts);
                        }}
                    >
                        New empty view...
                    </button>
                </svelte:fragment>
            </Dropdown>
        </div>
    {/if}
</div>

<style>
    .details {
        width: 100%;
        height: 100%;
        min-height: 0;
        max-height: 100%;
        overflow-y: auto;

        padding: 10px;
        border: 2px solid #666;
        border-radius: 10px;
    }

    div#centred-text {
        display: grid;
        place-items: center;
        gap: 5px;
    }

    div#selected-concerts-summary {
        display: flex;
        flex-direction: column;
        gap: 5px;
        width: 70%;
        margin-bottom: 20px;
    }

    p {
        margin: 0;
    }

    .italic {
        font-style: italic;
    }
</style>
