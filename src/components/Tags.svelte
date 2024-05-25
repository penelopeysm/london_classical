<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    import { allBooleanFilters } from "src/lib/filters";
    import Tag from "src/components/Tag.svelte";

    export let concert: Concert;
</script>

<div id="tags">
    {#each allBooleanFilters as boolFilter}
        {#if boolFilter.filterFunc(concert)}
            <Tag {boolFilter} />
        {/if}
    {/each}
</div>

<style>
    div#tags {
        display: flex;
        justify-content: flex-start;
        flex-wrap: wrap;
        gap: 5px;
        margin-bottom: 5px;
    }
</style>
