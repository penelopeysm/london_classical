<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    import {
        concertViews,
        filters,
        currentViewName,
        selectedConcertIndices,
        defaultViewName,
    } from "src/lib/stores";
    import { initialFilters } from "src/lib/filters";
    import FileSelector from "src/components/FileSelector.svelte";
    import Dropdown from "src/components/Dropdown.svelte";

    export let allConcerts: Concert[];
    export let shownIndices: number[];

    function setViewName(newViewName: string) {
        $currentViewName = newViewName;
    }

    function addEmptyView() {
        const newViewName = getNewViewName();
        if (newViewName === null) {
            return;
        }
        $concertViews.set(newViewName, []);
        $concertViews = new Map($concertViews); // Required to trigger store update
        $currentViewName = newViewName;
    }

    function addViewFromShownConcerts() {
        const newViewName = getNewViewName();
        if (newViewName === null) {
            return;
        }
        const shownConcerts = shownIndices.map((i) => allConcerts[i]);
        $filters = initialFilters;
        $concertViews.set(newViewName, shownConcerts);
        $concertViews = new Map($concertViews); // Required to trigger store update
        $currentViewName = newViewName;
    }

    function addViewFromSelectedConcerts() {
        const newViewName = getNewViewName();
        if (newViewName === null) {
            return;
        }
        const selectedConcerts = $selectedConcertIndices.map(
            (i) => allConcerts[i],
        );
        $filters = initialFilters;
        $concertViews.set(newViewName, selectedConcerts);
        $concertViews = new Map($concertViews); // Required to trigger store update
        $currentViewName = newViewName;
    }

    let fileSelectorMode: "select" | "error" | "hidden" = "hidden";
    function addViewFromJSON() {
        fileSelectorMode = "select";
    }

    function getNewViewName(): string | null {
        const newViewName = prompt("Enter a name for the new view");
        if (newViewName === null) {
            return null;
        }
        if (newViewName === "") {
            alert("Please enter a name");
            return null;
        }
        if ($concertViews.has(newViewName)) {
            alert("A view with that name already exists");
            return null;
        }
        return newViewName;
    }

    function deleteView(viewName: string) {
        window.confirm(`Really delete view "${viewName}"?`);
        $concertViews.delete(viewName);
        $concertViews = new Map($concertViews); // Required to trigger store update
        $currentViewName = defaultViewName;
    }

    function exportView(viewName: string) {
        let concertIds = notUndefined($concertViews.get(viewName)).map(
            (c) => c.id,
        );
        const obj: { [viewName: string]: string[] } = {};
        obj[viewName] = concertIds;
        const exportJson = JSON.stringify(obj);
        console.log(exportJson);
        console.log("Exporting view", viewName);

        if (window.showSaveFilePicker === undefined) {
            alert(
                "This feature is only available in browsers that support the File System Access API. Upgrade your browser (and if you're on Safari, use a different browser, please)",
            );
            return;
        }

        const filePickerOpts: SaveFilePickerOptions = {
            types: [
                {
                    description: "concert list",
                    accept: { "application/json": [".json"] },
                },
            ],
            suggestedName: "concerts.json",
        };
        showSaveFilePicker(filePickerOpts)
            .then((fileHandle) => {
                console.log("Writing to file", fileHandle.name);
                return fileHandle.createWritable();
            })
            .then((writableStream) => {
                writableStream.write(exportJson);
                writableStream.close();
            });
    }

    // Silence type errors
    function notUndefined<T>(value: T | undefined): T {
        if (value === undefined) {
            throw new Error("Value is undefined");
        }
        return value;
    }
</script>

<div class="view-list">
    {#each $concertViews.keys() as viewName}
        {@const allConcertsLength = notUndefined(
            $concertViews.get(viewName),
        ).length}
        {@const shownConcertsLength = shownIndices.length}
        <Dropdown
            hasOptions={viewName !== defaultViewName}
            selected={$currentViewName === viewName}
            on:mainButtonClick={() => setViewName(viewName)}
        >
            <span slot="text">
                {#if $currentViewName === viewName && shownConcertsLength !== allConcertsLength}
                    {viewName} ({shownConcertsLength}/{allConcertsLength})
                {:else}
                    {viewName} ({allConcertsLength})
                {/if}
            </span>
            <svelte:fragment slot="options">
                <button on:click={() => exportView(viewName)}
                    >Export view to JSON</button
                >
                <button on:click={() => deleteView(viewName)}
                    >Delete view</button
                >
            </svelte:fragment>
        </Dropdown>
    {/each}
    <Dropdown selected={false}>
        <span slot="text">Add new view</span>
        <svelte:fragment slot="options">
            <button on:click={addEmptyView}>New empty view</button>
            <button on:click={addViewFromShownConcerts}
                >... from currently shown concerts</button
            >
            <button on:click={addViewFromSelectedConcerts}
                >... from currently selected concerts</button
            >
            <button on:click={addViewFromJSON}>... from a file upload</button>
        </svelte:fragment>
    </Dropdown>
</div>
<FileSelector bind:mode={fileSelectorMode} />

<style>
    div.view-list {
        display: flex;
        flex-wrap: wrap;
        gap: 7px;
        align-items: baseline;
    }
</style>
