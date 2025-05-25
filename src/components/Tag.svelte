<script lang="ts">
    import { type BooleanFilter } from "src/lib/filters";

    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    function handleClick() {
        dispatch("clicked", {
            boolFilter: boolFilter,
        });
    }

    export let boolFilter: BooleanFilter;
    export let mode: "normal" | "canAdd" | "canRemove" = "normal";
</script>

<button
    class="tag"
    class:can-add={mode === "canAdd"}
    class:can-remove={mode === "canRemove"}
    style="background-color: {boolFilter.tagColor}"
    on:click={handleClick}
>
    <span>{boolFilter.tagName}</span>
</button>

<style>
    button.tag {
        font-family: inherit;
        padding: 0px 3px;
        border-radius: 5px;
        border: 1.5px solid transparent;
        margin: 0;
        color: white;
        width: max-content;
    }

    button.tag.can-remove,
    button.tag.can-add {
        cursor: pointer;
    }

    button.tag.can-remove > span::before {
        content: "− ";
    }

    button.tag.can-add > span::before {
        content: "+ ";
    }

    button.tag:hover {
        border: 1.5px solid black;
        transition: border 0.2s ease-in-out;
    }
</style>
