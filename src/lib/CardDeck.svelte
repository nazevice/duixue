<script>
    export let deckName;
    export let deckDescription;
</script>
<div>
    <h1>{deckName}</h1>
    <p>
        {#if deckDescription}
            {deckDescription}
        {:else}
            No description
        {/if}
    </p>
</div>