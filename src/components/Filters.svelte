<script lang="ts">
    import { type FiltersType, allBooleanFilters } from "src/lib/filters";
    import Tag from "src/components/Tag.svelte";

    export let filters: FiltersType;

    function toggleBooleanTag(
        event: CustomEvent<{ boolFilter: BooleanFilter }>,
    ) {
        const { boolFilter } = event.detail;

        if (filters.booleanTagNames.includes(boolFilter.tagName)) {
            filters.booleanTagNames = filters.booleanTagNames.filter(
                (tagName) => tagName !== boolFilter.tagName,
            );
        } else {
            filters.booleanTagNames = [
                ...filters.booleanTagNames,
                boolFilter.tagName,
            ];
        }
    }
</script>

<div class="filters">
    <h3>Filter concerts...</h3>
    <div class="horizontal-flex">
        <input
            id="search"
            type="text"
            placeholder="Search for a composer, performer, etc."
            bind:value={filters.searchTerm}
        />
        {#each allBooleanFilters as boolFilter}
            {#if filters.booleanTagNames.includes(boolFilter.tagName)}
                <Tag
                    {boolFilter}
                    mode="canRemove"
                    on:clicked={toggleBooleanTag}
                />
            {/if}
        {/each}
    </div>

    <span class="bold">Add a filter</span>
    <div class="horizontal-flex">
        {#each allBooleanFilters as boolFilter}
            {#if !filters.booleanTagNames.includes(boolFilter.tagName)}
                <Tag {boolFilter} mode="canAdd" on:clicked={toggleBooleanTag} />
            {/if}
        {/each}
    </div>
</div>

<style>
    h3 {
        margin: 0;
    }

    .filters {
        width: 100%;
        max-width: 100%;
        background-color: #f0f0f0;
        padding: 10px;
        border-radius: 0.5rem;
        border: 2px solid #666;

        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    input {
        font-family: inherit;
    }

    input#search {
        width: 300px;
    }

    .horizontal-flex {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        gap: 5px;
        align-items: baseline;
    }

    .bold {
        font-weight: bold;
    }
</style>
