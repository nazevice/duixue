<script>
    import { invoke } from "@tauri-apps/api/primitives"

    let formCard = {
    character: '',
    pinyin: '',
    translation: ''
    };

    const addCard = async () => {
        let greetMsg = await invoke("add_card", {card: formCard})
    }
</script>
<div class="w-9/12">
    <form on:submit|preventDefault={addCard}>
        <h1>Charakter</h1>
        <input class="border-2 border-indigo-500 w-full" bind:value={formCard.character} />
        <h1>Pinyin</h1>
        <input class="border-2 border-indigo-500 w-full" bind:value={formCard.pinyin} />
        <h1>Übersetzung</h1>
        <input class="border-2 border-indigo-500 w-full" bind:value={formCard.translation} />
        <button class="bg-indigo-500 p-2 rounded-lg w-full" type="submit">Add</button>
    </form>
</div>