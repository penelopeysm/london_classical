<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    import {
        concertViews,
        filters,
        currentViewName,
        selectedConcertIndices,
    } from "src/lib/stores";
    import { initialFilters } from "src/lib/filters";
    import FileSelector from "src/components/FileSelector.svelte";

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
        $concertViews.delete(viewName);
        $concertViews = new Map($concertViews); // Required to trigger store update
        $currentViewName = "All";
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
        <div class="dropdown-trigger">
            <button
                class="view-button dropdown-button"
                class:active={$currentViewName === viewName}
                on:click={() => setViewName(viewName)}
            >
                {#if $currentViewName === viewName && shownConcertsLength !== allConcertsLength}
                    {viewName} ({shownConcertsLength}/{allConcertsLength})
                {:else}
                    {viewName} ({allConcertsLength})
                {/if}
                {#if viewName !== "All"}
                    <span class="smol">▼</span>
                    <div class="dropdown-options">
                        <button on:click={() => exportView(viewName)}
                            >Export view to JSON</button
                        >
                        <button on:click={() => deleteView(viewName)}
                            >Delete view</button
                        >
                    </div>
                {/if}
            </button>
        </div>
    {/each}
    <div class="dropdown-trigger">
        <button class="view-button dropdown-button">
            Add new view <span class="smol">▼</span>
            <div class="dropdown-options">
                <button on:click={addEmptyView}>New empty view</button>
                <button on:click={addViewFromShownConcerts}
                    >... from currently shown concerts</button
                >
                <button on:click={addViewFromSelectedConcerts}
                    >... from currently selected concerts</button
                >
                <button on:click={addViewFromJSON}
                    >... from a file upload</button
                >
            </div>
        </button>
    </div>
</div>
<FileSelector bind:mode={fileSelectorMode} />

<style>
    button.view-button {
        background-color: #f0f0f0;
        border: 1px solid #ccc;
        border-radius: 5px;
        padding: 5px;

        transition: all 0.3s;
        transition-property: background-color, border-color, box-shadow;
    }

    button.active {
        background-color: #c1eaf5;
        border-color: #32aecf;
        box-shadow: 0 0 1px #32aecf;
    }

    div.view-list {
        display: flex;
        flex-wrap: wrap;
        gap: 7px;
        align-items: baseline;
    }

    button.dropdown-button {
        position: relative;
    }

    div.dropdown-options {
        display: none;
    }

    button.dropdown-button:hover > div.dropdown-options {
        display: flex;
        flex-direction: column;
        gap: 0px;
        position: absolute;
        top: 26.5px;
        left: -1px;
        width: max-content;
        z-index: 1;
    }

    div.dropdown-options button {
        background-color: #f0f0f0;
        border: 1px solid #ccc;
        border-radius: 5px;
        padding: 5px;
        margin: 0;
        width: 100%;
        text-align: left;
    }

    div.dropdown-options button:hover {
        background-color: #ddd;
    }

    span.smol {
        font-size: 0.7em;
        margin-left: 2px;
    }
</style>
