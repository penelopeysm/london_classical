<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    import ConcertList from "src/components/ConcertList.svelte";
    import ViewList from "src/components/ViewList.svelte";
    import {
        concerts,
        filters,
        currentViewName,
        selectedConcertIndices,
    } from "src/lib/stores";
    import { getPassingIndices } from "src/lib/filters";

    let allConcerts: Concert[] = $concerts.get($currentViewName) as Concert[];
    let shownIndices: number[] = getPassingIndices(allConcerts, $filters);

    $: {
        // Calculate the indices of the concerts that should be shown
        allConcerts = $concerts.get($currentViewName) as Concert[];
        shownIndices = getPassingIndices(allConcerts, $filters);
        // Clear selected concerts when filters are changed (or allConcerts for that matter)
        $selectedConcertIndices = [];
    }
</script>

<div class="overview">
    <ViewList bind:allConcerts bind:shownIndices />
    <ConcertList bind:allConcerts bind:shownIndices />
</div>

<style>
    div.overview {
        display: flex;
        flex-direction: column;
        gap: 10px;
        min-height: 0;
        height: 100%;
        max-height: 100%;
    }
</style>
