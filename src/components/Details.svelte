<script lang="ts">
    import { type HashedConcert } from "src/utils";
    import SelectedConcertDetails from "src/components/SelectedConcertDetails.svelte";

    export let selectedConcerts: HashedConcert[];

    let copyButton: HTMLButtonElement;
    function copyTextToClipboard() {
        navigator.clipboard.writeText(shareCode);
        let w = copyButton.offsetWidth;
        copyButton.style.width = w + "px";
        copyButton.textContent = "Copied!";
        setTimeout(() => {
            copyButton.textContent = "Copy to clipboard";
            copyButton.style.width = "";
        }, 1000);
    }

    let shareCode: string;
    $: {
        shareCode = selectedConcerts.map((concert) => concert.hash).join(",");
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
                        <a href={concert.url}>
                            {concert.title}
                        </a>
                    </p>
                {/each}
            </div>
            <p>
                Share this list of concerts with somebody by sending them the
                text below.
                (They don't have any way of inputting this code into the website
                yet, but one day...)
            </p>
            <p></p>
            <p class="code">{shareCode}</p>
            <button bind:this={copyButton} on:click={copyTextToClipboard}>
                Copy to clipboard
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

    .code {
        font-family: monospace;
        width: max-content;
        max-width: 80%;
    }

    button {
        margin-top: 5px;
        font-family: inherit;
    }
</style>
