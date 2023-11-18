<script>
    import { invoke } from "@tauri-apps/api/primitives"
    
    let file;

    function readFileAsBase64() {
    if (!file) return;   
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(reader.result);
        reader.onerror = reject;
        reader.readAsDataURL(file);
    });
}

    let formCard = {
    character: '',
    pinyin: '',
    translation: '',
    file_base64: ''
    };

    const addCard = async () => {
        formCard.file_base64 = await readFileAsBase64(file);
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
        <label for="file">Upload your file</label>
        <input type="file" on:change="{(e) => (file = e.target.files[0])}" />
        <button class="bg-indigo-500 p-2 rounded-lg w-full" type="submit">Add</button>
    </form>
</div>