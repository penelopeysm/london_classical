<script lang="ts">
    export let selected: boolean = false;
    export let hasOptions: boolean = true;
    export let alignment: "left" | "right" = "left";

    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();
    function mainButtonClick() {
        dispatch("mainButtonClick", {});
    }
</script>

<div class="dropdown-trigger">
    <button class="dropdown-button" class:selected on:click={mainButtonClick}>
        <slot name="text" />
        {#if hasOptions}
            <span class="smol">▼</span>
            <div
                class="dropdown-options"
                class:left={alignment === "left"}
                class:right={alignment === "right"}
            >
                <slot name="options" />
            </div>
        {/if}
    </button>
</div>

<style>
    button.dropdown-button {
        position: relative;
        background-color: #f0f0f0;
        border: 1px solid #ccc;
        border-radius: 5px;
        padding: 5px;
    }

    button.dropdown-button.selected {
        background-color: #c1eaf5;
        border-color: #32aecf;
        box-shadow: 0 0 1px #32aecf;
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
        width: max-content;
        z-index: 1;
        align-items: stretch;
    }

    button.dropdown-button:hover > div.dropdown-options.left {
        left: -1px;
    }

    button.dropdown-button:hover > div.dropdown-options.right {
        right: 1px;
    }

    div.dropdown-options :global(button) {
        background-color: #f0f0f0;
        border: 1px solid #ccc;
        border-bottom: none;
        padding: 5px;
        margin: 0;
        text-align: left;  /* Regardless of alignment, text should always be on left */
    }

    div.dropdown-options :global(button:last-child) {
        border-bottom: 1px solid #ccc;
    }

    div.dropdown-options :global(button:hover) {
        background-color: #ddd;
    }

    span.smol {
        font-size: 0.7em;
        margin-left: 2px;
    }
</style>
