<script lang="ts">
    import Tags from "src/components/Tags.svelte";
    import { type HashedConcert, formatDate, getPriceString } from "src/utils";

    export let concerts: HashedConcert[];
    export let selectedConcertHashes: string[];

    // Event handler when a concert is clicked. The behaviour is chosen to
    // provide as intuitive a UI as possible
    function addOrRemove(event: MouseEvent, hash: string) {
        if (selectedConcertHashes.includes(hash)) {
            if (event.shiftKey) {
                selectedConcertHashes = selectedConcertHashes.filter(
                    (h) => h !== hash,
                );
            } else {
                if (selectedConcertHashes.length === 1) {
                    selectedConcertHashes = [];
                } else {
                    selectedConcertHashes = [hash];
                }
            }
        } else {
            if (event.shiftKey) {
                selectedConcertHashes = [...selectedConcertHashes, hash];
            } else {
                selectedConcertHashes = [hash];
            }
        }
    }
</script>

<div class="overview">
    {#each concerts as concert}
        <button
            class="concert"
            class:active={selectedConcertHashes.includes(concert.hash)}
            class:wigmoreU35={concert.is_wigmore_u35}
            on:click={(event) => addOrRemove(event, concert.hash)}
        >
            <Tags {concert} />
            <h3>{concert.title}</h3>
            {#if concert.subtitle !== null}
                <h4>{concert.subtitle}</h4>
            {/if}
            <p>
                {formatDate(new Date(concert.datetime))}
                |
                {getPriceString(concert)}
            </p>
        </button>
    {/each}
</div>

<style>
    .overview {
        display: flex;
        flex-direction: column;
        gap: 10px;
        width: 100%;
        height: 100%;
        overflow-y: auto;
        padding: 0 15px 0 0;
    }

    .concert {
        border: 2px solid #666;
        padding: 10px;
        border-radius: 5px;
        width: 100%;

        font-family: inherit;
        text-align: left;
        cursor: pointer;

        transition: border-color 0.3s;
        transition: background-color 0.3s;
        transition: margin-left 0.3s;
    }

    .active {
        border-color: #a6628f;
        background-color: #f5e9f1;
    }

    .concert > * {
        margin: 2px 0;
    }
</style>
