<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/primitives";
    import CardInput from "$lib/CardInput.svelte";
    import Greet from "$lib/Greet.svelte";
    import CardDeck from "$lib/CardDeck.svelte";
  
    let cardDecks = [];
  
    onMount(async () => {
      const data = await invoke("get_card_decks");
      cardDecks = JSON.parse(data);
    });
  
    const handleClick = () => {
      console.log("test");
    }
  </script>
  <main class="container">
    {console.log(cardDecks)}
    {#each cardDecks as cardDeck}
      <CardDeck deckName={cardDeck.name} deckDescription={cardDeck.description} />
      <button on:click={handleClick}>
        {cardDeck.name}
        {#if cardDeck.description}
          - {cardDeck.description}
        {:else}
          - No description
        {/if}
      </button>
    {/each}
    <h1>Cards</h1>
    <div class="w-96 bg-white shadow rounded">w-96</div>
    <button
      class="bg-blue-700 px-4 py-2 text-white hover:bg-sky-800 sm:px-8 sm:py-3"
    />
    <CardInput />
    <Greet />
  </main>