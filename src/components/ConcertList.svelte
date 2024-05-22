<script lang="ts">
    import Tags from "src/components/Tags.svelte";
    import { type Concert } from "src/lib/bindings/Concert";
    import { formatDate, getPriceString } from "src/lib/utils";
    import { selectedConcertIndices } from "src/lib/stores";

    export let allConcerts: Concert[];
    export let shownIndices: number[];

    // Event handler when a concert is clicked. The behaviour is chosen to
    // provide as intuitive a UI as possible
    function selectOrDeselect(event: MouseEvent, idx: number) {
        if ($selectedConcertIndices.includes(idx)) {
            if (event.shiftKey) {
                $selectedConcertIndices = $selectedConcertIndices.filter(
                    (i) => i !== idx,
                );
            } else {
                if ($selectedConcertIndices.length === 1) {
                    $selectedConcertIndices = [];
                } else {
                    $selectedConcertIndices = [idx];
                }
            }
        } else {
            if (event.shiftKey) {
                $selectedConcertIndices = [...$selectedConcertIndices, idx];
            } else {
                $selectedConcertIndices = [idx];
            }
        }
    }
</script>

<div class="concert-list">
    {#each shownIndices as idx}
        {@const concert = allConcerts[idx]}
        <button
            class="concert"
            class:active={$selectedConcertIndices.includes(idx)}
            on:click={(event) => selectOrDeselect(event, idx)}
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
    .concert-list {
        display: flex;
        flex-direction: column;
        gap: 10px;
        width: 100%;
        overflow-y: auto;
        padding: 0 15px 0 0;
    }

    .concert {
        border: 2px solid #666;
        padding: 10px;
        border-radius: 5px;
        width: calc(100% - 5px); /* Leave some space for scrollbar on some systems */

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
