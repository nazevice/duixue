<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from "@tauri-apps/api/primitives"
    import CardInput from './lib/CardInput.svelte';
    import Greet from './lib/Greet.svelte'
    
    onMount(async() => {
      const data = await invoke('get_card_decks');
      console.log(data)
    })
</script>

<main class="container">
  <h1>Cards</h1>
  <div class="w-96 bg-white shadow rounded">
    w-96
  </div>
  <button class="bg-blue-700 px-4 py-2 text-white hover:bg-sky-800 sm:px-8 sm:py-3"></button>
  <CardInput />
  <Greet />
</main>