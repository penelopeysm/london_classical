<script lang="ts">
    import { type Concert } from "src/lib/bindings/Concert";
    export let concert: Concert;
</script>

<div id="tags">
    {#if concert.venue === "Wigmore Hall"}
        <span class="tag wigmore">Wigmore Hall</span>
    {/if}
    {#if concert.is_wigmore_u35}
        <span class="tag wigmore-u35">£5 for U35</span>
    {/if}
</div>

<style>
    div#tags {
        display: flex;
        justify-content: flex-start;
        gap: 5px;
        margin-bottom: 5px;
    }

    span.tag {
        padding: 2px 5px;
        border-radius: 5px;
        margin: 0;
    }

    span.wigmore {
        background-color: #17a8ad;
        color: white;
    }

    span.wigmore-u35 {
        background-color: #3694cf;
        color: white;
    }
</style>
