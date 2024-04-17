<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    import Tags from "src/lib/Tags.svelte";
    import { formatDate, getPriceString } from "src/utils";

    export let concerts: Concert[];
    export let selectedConcert: Concert | null;
</script>

<div class="overview">
    {#each concerts as concert}
        <button
            class="concert"
            class:active={selectedConcert === concert}
            class:wigmoreU35={concert.is_wigmore_u35}
            on:click={() => {
                selectedConcert = selectedConcert === concert ? null : concert;
            }}
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
        margin-left: 15px;
    }

    .concert > * {
        margin: 2px 0;
    }
</style>
