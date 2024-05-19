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
        padding: 2px 5px;
        border-radius: 5px;
        margin: 0;
        color: white;
        border: none;
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
</style>
