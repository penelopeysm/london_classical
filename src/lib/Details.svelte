<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    import Tags from "src/lib/Tags.svelte";
    import { formatDate, getPriceString } from "src/utils";

    export let selectedConcert: Concert | null;
</script>

<div class="details">
    {#if selectedConcert !== null}
        <div id="selected">
            <Tags concert={selectedConcert} />
            <h2>
                {selectedConcert.title}
                {#if selectedConcert.subtitle}
                    — {selectedConcert.subtitle}
                {/if}
            </h2>
            <p>
                {formatDate(new Date(selectedConcert.datetime))}
                |
                {getPriceString(selectedConcert)}
                <br />
                <a href={selectedConcert.url}>Link to concert</a>
                {#if selectedConcert.programme_pdf_url}
                    | <a href={selectedConcert.programme_pdf_url}
                        >Link to programme (PDF)</a
                    >
                {/if}
            </p>

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

            <h3>Programme</h3>
            {#if selectedConcert.pieces.length === 0}
                None provided.
            {:else}
                <div class="two-col-grid">
                    {#each selectedConcert.pieces as piece}
                        <span>{piece.composer}</span><span
                            >{@html piece.title}</span
                        >
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
    {:else}
        <div id="none-selected">
            <h2>No concert selected</h2>
            <p>Select a concert from the list on the left to view details :)</p>
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

    #selected > *:first-child {
        margin-top: 0;
    }

    #selected > *:last-child {
        margin-bottom: 0;
    }

    h3 {
        margin-bottom: 5px;
    }

    div#none-selected {
        display: grid;
        place-items: center;
    }

    div.two-col-grid {
        display: grid;
        grid-template-columns: max-content 1fr;
        gap: 4px 30px;
    }

    div#description > *:first-child {
        margin-top: 0;
    }
</style>
