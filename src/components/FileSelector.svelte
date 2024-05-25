<script lang="ts">
    import {
        viewFromJson,
        concertViews,
        currentViewName,
    } from "src/lib/stores";

    export let mode: "select" | "error" | "hidden" = "select";

    let fileSelectorInput: HTMLInputElement;

    let fileName: string | null = null;
    let fileContents: string | null = null;
    let highlighted = false;
    let errorMessage: string | null = null;

    function handleDrop(event: DragEvent) {
        event.preventDefault();
        highlighted = false;

        if (event.dataTransfer) {
            const file = event.dataTransfer.files[0];
            fileName = file.name;

            const reader = new FileReader();
            reader.onload = (e) => {
                fileContents = e.target!.result as string;
            };
            reader.readAsText(file);
        }
    }

    function handleInput() {
        if (fileSelectorInput.files) {
            const file = fileSelectorInput.files[0];
            fileName = file.name;

            const reader = new FileReader();
            reader.onload = (e) => {
                fileContents = e.target!.result as string;
            };
            reader.readAsText(file);
        }
    }

    function close() {
        mode = "hidden";
        fileName = null;
        fileContents = null;
    }

    function submit() {
        if (fileName === null) {
            return;
        }
        try {
            const newViews = viewFromJson(fileContents!);
            console.log(newViews);
            for (let [viewName, concerts] of newViews) {
                // Automatically rename views if they already exist
                while ($concertViews.has(viewName)) {
                    viewName = viewName + "*";
                }
                $concertViews.set(viewName, concerts);
                $concertViews = new Map($concertViews); // Required to trigger store update
                $currentViewName = viewName;
            }
            close();
        } catch (e) {
            mode = "error";
            errorMessage = e.message;
        }
    }
</script>

{#if mode === "select"}
    <button id="background-close" on:click={close}>
        <div id="background-close" />
    </button>

    <div id="file-select">
        <button
            id="file-drop"
            class:highlighted
            on:drop={handleDrop}
            on:dragover={(e) => {
                e.preventDefault();
                highlighted = true;
            }}
            on:dragleave={() => {
                highlighted = false;
            }}
            on:click={() => fileSelectorInput.click()}
        >
            <p>Drag a file here, or click to select a file</p>
            {#if fileContents === null}
                <p class="greyed">No file selected...</p>
            {:else}
                <p class="green">✅ {fileName}</p>
            {/if}
        </button>
        <input
            type="file"
            id="file-selector"
            bind:this={fileSelectorInput}
            on:change={handleInput}
        />

        <button class="larger-text bold" on:click={submit}
            >Load view from file</button
        >
        <button class="larger-text" on:click={close}>Close</button>
    </div>
{:else if mode === "error"}
    <button id="background-close" on:click={close}>
        <div id="background-close" />
    </button>

    <div class="error">
        <p class="bold">Sorry! There was an error loading the file:</p>
        <p>{errorMessage}</p>
        <button
            class="larger-text"
            on:click={() => {
                mode = "select";
            }}>Try again</button
        >
        <button class="larger-text" on:click={close}>Close</button>
    </div>
{/if}

<svelte:window
    on:keydown={(e) => {
        if (e.key === "Escape") {
            close();
        }
    }}
/>

<style>
    button#background-close {
        display: contents;
    }

    div#background-close {
        position: absolute;
        height: 100vh;
        width: 100vw;
        top: 0;
        left: 0;
        background-color: rgba(0, 0, 0, 0.5);
        z-index: 1;
        cursor: pointer;
    }

    div#file-select,
    div.error {
        position: absolute;
        z-index: 2;
        padding: 20px;
        border-radius: 10px;
        height: max-content;
        width: max-content;
        top: 50vh;
        left: 50vw;
        transform: translate(-50%, -50%);
        background-color: #e9e9e9;
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    div.error {
        color: #eb4281;
    }

    input#file-selector {
        display: none;
    }

    button#file-drop {
        border: 3px dashed #ccc;
        border-radius: 5px;
        padding: 20px;
        text-align: center;
        font-size: 110%;
        cursor: pointer;
        background-color: #f0f0f0;
        display: flex;
        flex-direction: column;
        gap: 10px;
        align-items: center;
    }

    button#file-drop.highlighted {
        border-color: #32a852;
        background-color: #e9f7ed;
    }

    p {
        margin: 0;
    }

    p.greyed,
    p.green {
        max-width: 230px;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
    }

    p.greyed {
        color: #888;
    }

    p.green {
        color: #32a852;
    }

    .bold {
        font-weight: bold;
    }

    .larger-text {
        width: 100%;
        font-size: 110%;
    }
</style>
