<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    import SelectedConcertDetails from "src/components/SelectedConcertDetails.svelte";
    import {
        concerts,
        currentViewName,
        selectedConcertIndices,
    } from "src/lib/stores";

    let allConcerts: Concert[] = $concerts.get($currentViewName) as Concert[];
    let selectedConcerts: Concert[] = allConcerts.filter((_, idx) =>
        $selectedConcertIndices.includes(idx),
    );

    $: {
        allConcerts = $concerts.get($currentViewName) as Concert[];
        selectedConcerts = allConcerts.filter((_, idx) =>
            $selectedConcertIndices.includes(idx),
        );
    }

    function exportSelection() {
        alert("TODO: Export selected concerts to file");
    }

    function makeNewView() {
        alert("TODO: Create new view with selected concerts");
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

            <p>
                Don&rsquo;t click these buttons, they don&rsquo;t do anything
                yet. You&rsquo;ll just get an annoying popup.
            </p>

            <button on:click={exportSelection}>
                TODO: Export selected concerts to file
            </button>

            <button on:click={makeNewView}>
                TODO: Create new view with selected concerts
            </button>
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

    button {
        margin-top: 5px;
        font-family: inherit;
    }
</style>
