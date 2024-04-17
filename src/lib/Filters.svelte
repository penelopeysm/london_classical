<script lang="ts">
    import { type FiltersType } from "src/lib/filters";
    export let filters: FiltersType;
    $: {
        console.log(filters);
    }
</script>

<div class="filters">
    <input
        id="search"
        type="text"
        placeholder="Search for a composer, performer, etc."
        bind:value={filters.searchTerm}
    />

    <div id="u35">
        <input
            type="checkbox"
            id="wigmoreU35"
            name="wigmoreU35"
            bind:checked={filters.wigmoreU35}
        />
        <label for="wigmoreU35">£5 Wigmore Hall under-35 tickets only?</label>
    </div>
</div>

<style>
    .filters {
        width: 100%;
        background-color: #f0f0f0;
        padding: 10px;
        border-radius: 0.5rem;
        border: 2px solid #666;

        display: flex;
        flex-direction: column;
        gap: 5px;
    }

    input {
        font-family: inherit;
    }

    input#search {
        width: 300px;
    }
</style>
