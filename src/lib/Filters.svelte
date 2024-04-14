<script lang="ts">
    export let searchTerm: string;
</script>

<div class="filters">
    <input
        type="text"
        placeholder="Search for a composer, performer, etc."
        bind:value={searchTerm}
    />
</div>

<style>
    .filters {
        width: 100%;
        background-color: #f0f0f0;
        padding: 1rem;
        border-radius: 0.5rem;
    }

    input {
        font-family: inherit;
        width: 300px;
    }
</style>
