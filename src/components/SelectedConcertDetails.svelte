<script lang="ts">
    import Tags from "src/components/Tags.svelte";
    import { formatDate, getPriceString } from "src/lib/utils";
    import { type Concert } from "src/lib/bindings/Concert";
    import { concertViews } from "src/lib/stores";
    import Dropdown from "src/components/Dropdown.svelte";

    export let selectedConcert: Concert;

    function getValidViews(concert: Concert): string[] {
        let validViews = [];
        for (const [view, concerts] of $concertViews.entries()) {
            if (concerts.find((c) => c.id === concert.id) === undefined) {
                validViews.push(view);
            }
        }
        return validViews;
    }

    // These are handled in the parent component to avoid code duplication
    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();
    function addToView(concert: Concert, viewName: string) {
        dispatch("add", { concerts: [concert], viewName });
    }
    function addToNewView(concert: Concert) {
        dispatch("addNew", { concerts: [concert] });
    }
</script>

<div id="selected">
    <h2>
        {selectedConcert.title}
        {#if selectedConcert.subtitle}
            — {selectedConcert.subtitle}
        {/if}
    </h2>
    <div class="left-and-right">
        <Tags concert={selectedConcert} />
        <Dropdown alignment="right">
            <span slot="text">Add to view</span>
            <svelte:fragment slot="options">
                {#each getValidViews(selectedConcert) as view}
                    <button
                        on:click={() => {
                            addToView(selectedConcert, view);
                        }}
                    >
                        {view}
                    </button>
                {/each}
                <button
                    on:click={() => {
                        addToNewView(selectedConcert);
                    }}
                >
                    New empty view...
                </button>
            </svelte:fragment>
        </Dropdown>
    </div>
    <div>
        {formatDate(new Date(selectedConcert.datetime))}
        |
        {getPriceString(selectedConcert)}
        <br />
        <a href={selectedConcert.url} target="_blank">Link to concert</a>
        {#if selectedConcert.programme_pdf_url}
            | <a href={selectedConcert.programme_pdf_url} target="_blank"
                >Link to programme (PDF)</a
            >
        {/if}
    </div>

    <div>
        <h3>Performer(s)</h3>
        {#if selectedConcert.performers.length === 0}
            None listed.
        {:else}
            <div class="two-col-grid">
                {#each selectedConcert.performers as performer}
                    <span>{performer.name}</span>
                    <span
                        >{performer.instrument
                            ? performer.instrument
                            : ""}</span
                    >
                {/each}
            </div>
        {/if}
    </div>

    <h3>Programme</h3>
    {#if selectedConcert.pieces.length === 0}
        None provided.
    {:else}
        <div class="two-col-grid">
            {#each selectedConcert.pieces as piece}
                <span>{piece.composer}</span><span>{@html piece.title}</span>
            {/each}
        </div>
    {/if}

    <h3>Description</h3>
    {#if selectedConcert.description}
        <div id="description">
            {#each selectedConcert.description.split("\n") as paragraph}
                <p>{paragraph}</p>
            {/each}
        </div>
    {:else}
        None provided.
    {/if}
</div>

<style>
    #selected {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    h2 {
        margin: 0;
    }

    h3 {
        margin-top: 0;
        margin-bottom: 5px;
    }

    div.left-and-right {
        display: grid;
        grid-template-columns: 1fr max-content;
        align-items: baseline;
    }

    div.two-col-grid {
        display: grid;
        grid-template-columns: auto 1fr;
        gap: 4px 30px;
    }

    div#description > *:first-child {
        margin-top: 0;
    }
</style>
