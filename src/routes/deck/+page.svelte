<script>

    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { deckId } from "../../stores/deck";
    import { invoke } from "@tauri-apps/api/primitives";

    let cards = [];
    onMount(async () => {
        const data = await invoke("get_cards", {deckId: $deckId});
        cards = JSON.parse(data);
        console.log(cards);
    })
</script>
<div>
    test
    <button on:click={() => goto("/")}>abc</button>
</div>