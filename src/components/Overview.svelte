<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    import ConcertList from "src/components/ConcertList.svelte";
    import ViewList from "src/components/ViewList.svelte";
    import {
        concertViews,
        filters,
        currentViewName,
        selectedConcertIds,
    } from "src/lib/stores";
    import { getPassingIds, type FiltersType } from "src/lib/filters";

    let allConcerts: Concert[] = $concertViews.get(
        $currentViewName,
    ) as Concert[];
    let shownIds: string[] = getPassingIds(allConcerts, $filters);

    // Whenever either the view name changes or the filters are updated, update the UI,
    // and reset the selected concert IDs.
    // The reason why we explicitly take currentViewName and filters as arguments is that
    // we can use them in the $: block to trigger the updateUI function
    // whenever these variables change.
    // In contrast, we don't need to do this when $concertViews itself gets updated, so we
    // just use its value directly inside the function.
    // Hacky!
    function updateUI(currentViewName1: string, filters1: FiltersType) {
        allConcerts = $concertViews.get(currentViewName1) as Concert[];
        shownIds = getPassingIds(allConcerts, filters1);
        $selectedConcertIds = [];
    }

    $: {
        updateUI($currentViewName, $filters);
    }
</script>

<div class="overview">
    <ViewList bind:allConcerts bind:shownIds />
    <ConcertList bind:allConcerts bind:shownIds />
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
