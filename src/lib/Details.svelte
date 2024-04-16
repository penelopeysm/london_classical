<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";

    export let selectedConcert: Concert | null;
</script>

<div class="details">
    {#if selectedConcert !== null}
        <h3>{selectedConcert.title}</h3>
        {#if selectedConcert.subtitle}
            <h4>{selectedConcert.subtitle}</h4>
        {/if}
        <p>
            {selectedConcert.datetime.toLocaleString()} at {selectedConcert.venue}
        </p>

        <p>
            <a href={selectedConcert.url}>Link to concert</a>
            {#if selectedConcert.programme_pdf_url}
                | <a href={selectedConcert.programme_pdf_url}
                    >Link to programme (PDF)</a
                >
            {/if}
        </p>

        {#if selectedConcert.performers.length === 0}
            <p>No performers listed. (This is a placeholder!)</p>
        {:else}
            <h4>Performer{selectedConcert.performers.length > 1 ? "s" : ""}</h4>
            {#each selectedConcert.performers as performer}
                <p>{performer}</p>
            {/each}
        {/if}

        {#if selectedConcert.pieces.length === 0}
            <p>No programme provided.</p>
        {:else}
            <h4>Programme</h4>
            <div class="programme">
                {#each selectedConcert.pieces as piece}
                    <span>{piece.composer}</span><span>{piece.title}</span>
                {/each}
            </div>
        {/if}

        {#if selectedConcert.description}
            <h4>Description</h4>
            <p>{@html selectedConcert.description}</p>
        {/if}
    {:else}
        <p>No concert selected</p>
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
        border: 2px solid #000;
        border-radius: 10px;
    }

    .details > *:first-child {
        margin-top: 0;
    }

    .details > *:last-child {
        margin-bottom: 0;
    }

    div.programme {
        display: grid;
        grid-template-columns: max-content 1fr;
        gap: 4px 30px;
    }
</style>
