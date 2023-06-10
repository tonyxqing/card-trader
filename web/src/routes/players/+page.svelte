<script lang="ts">
    import FaTrash from 'svelte-icons/fa/FaTrash.svelte';
    import FaEdit from 'svelte-icons/fa/FaEdit.svelte';
    import FaCheck from 'svelte-icons/fa/FaCheck.svelte';
    import FaTimes from 'svelte-icons/fa/FaTimes.svelte';
    import FaUserPlus from 'svelte-icons/fa/FaUserPlus.svelte'
    import FaRegWindowRestore from 'svelte-icons/fa/FaRegWindowRestore.svelte'
    interface Skill {
        level: number, 
        experience: number,
    };
    interface Skills {
        attack: Skill,
        defense: Skill,
        strength: Skill,
        hitpoints: Skill,
    }


    type Card = {
        id: string,
        name: string,
        element: string,
        image: number[],
        imageUrl: string,
        skills: Skills,
        rarity: string,
        card_style: string,
        wear: number,
        owner_id: string,
    }
    
    type Player = { 
        name: string, 
        id: string, 
        date_created: string, 
        last_updated: string,
        cards: Card[] 
    }

    async function get_players() {
        const req = await fetch("http://localhost:8080/players", {
            method: "GET",
        });
        const data = await req.json();
        players = data;
    }

    async function get_player(player_id: string) {
        const req = await fetch(`http://localhost:8080/players/${player_id}`, {
            method: "GET",
        });
        const data = await req.json();
        return data;
    }

    async function add_card(player_id: string, name: string, files: any) {
        const req = await fetch(`http://localhost:8080/players/${player_id}/cards`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({name, image: files}),
        })
        const data = await req.json();
        selected_cards = [...selected_cards, data];
        console.log(selected_cards, data);
        return data;

    }
    async function fetch_one_card(card_id: string) {
        const req = await fetch(`http://localhost:8080/cards/${card_id}`, {
            method: "GET",
        })

        const data = await req.json();
        const imageUrl = getImageUrl(data.image);
        data.imageUrl = imageUrl;
        return data;
    }

    function getImageUrl(image: number[]) {
        const byteArray = new Uint8Array(image);
    
        // Convert the byte array to a Blob
        const blob = new Blob([byteArray], { type: 'image/jpeg' });

        // Create an object URL for the Blob
        return URL.createObjectURL(blob);
    }

    async function fetch_player_cards(id: string) {
        const req = await fetch(`http://localhost:8080/players/${id}/cards`, {
            method: "GET",
        });
        const data = await req.json();

        for (let d in data) {
            const imageUrl = getImageUrl(data[d].image);
            data[d].imageUrl = imageUrl;
        }
        selected_cards = data;
    }
    async function delete_card(card_id: string) {
        const req = await fetch(`http://localhost:8080/cards/${card_id}`, {
            method: "DELETE",
        })
    }

    async function edit_card(card_id: string, card: Card) {
        const req = await fetch(`http://localhost:8080/cards/${card_id}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                name: card.name,
                image: card.image,
                skills: card.skills,
                element: card.element,
                owner_id: card.owner_id,
            }),
        })
    }

    async function add_player(name: string) {
        const req = await fetch("http://localhost:8080/players", {
            method: "POST",
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ name })
        });
        get_players()
    }
    async function delete_player(id: string) {
        const req = await fetch(`http://localhost:8080/players/${id}`, {
            method: "DELETE",
        });
        get_players()
    }

    async function edit_player(id: string, obj: Partial<Player>) {
        const response = await fetch(`http://localhost:8080/players/${id}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(obj),
        })
        get_players()

    }

    get_players();
    let selected_files: any = [];
    let player_name = "";
    let card_name = "";
    let selected_player: Player | null;
    let selected_cards: Card[];
    let players: Player[] = [];
    let files: any;
    let files_image: number[];
    let editing_player_index: number | null;
    let editing_card_index: number | null;

    $: console.log(selected_cards);
</script>

<div class="table-container">
    {#if players.length > 0}
    <div class="player_row">    
        <p>Name</p>
        <p>ID</p>
        <p>Date Created</p>
        <p>Last Updated</p>
        <p>delete</p>
        <p>edit</p>    
    </div>
    {#each players as player, i}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="player_row" on:click={async () => {selected_player = await get_player(player.id); if (selected_player) {await fetch_player_cards(selected_player.id)}}}>    
        {#if i === editing_player_index}
        <span>
            <input bind:value={player.name}>
        </span>
        <p>{player.id}</p>
        <p>{player.date_created}</p>
        <p>{player.last_updated}</p>
        {:else}
        <p>{player.name}</p>
        <p>{player.id}</p>
        <p>{player.date_created}</p>
        <p>{player.last_updated}</p>
        {/if}    
        <span class="icon_button" on:click={async (e) => {if (player.id === selected_player?.id) {await delete_player(player.id); selected_cards = []; selected_player = null} else {await delete_player(player.id)} e.stopPropagation();}}>
                <FaTrash/>
        </span>
        {#if i === editing_player_index}
        <div>
            <span class="editing_buttons">
                <span class="icon_button" on:click={(e) => {edit_player(player.id, player); editing_player_index = null; e.stopPropagation();}}>
                    <FaCheck/>
                </span>
                <span class="icon_button" on:click={(e) => {editing_player_index = null; e.stopPropagation();}}>
                    <FaTimes/>
                </span>
            </span>
        </div>
        {:else}
        <span class="icon_button" on:click={(e) => {editing_player_index = i; e.stopPropagation();}}>
            <FaEdit/>
        </span>
        {/if}
        </div>
    {/each}
    {/if}
    <div class="player_row">
        <span>
            <input type="text" bind:value={player_name}>
        </span>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <span class="icon_button stack" on:click={() => add_player(player_name)}>
            <FaUserPlus/> Add Player
        </span>
    </div>
</div>
<div class="spacer"/>
<div class="selected_player_table">
{#if selected_player}
<div class="selected_player_row">
    <p>Player Name</p>
    <p>Player ID</p>
</div>
    <div class="selected_player_row">
        <p>{selected_player.name}</p>
        <p>{selected_player.id}</p>
    </div>
    {#if selected_cards}
    <div class="selected_player_row">
        <p>Card Name</p>
        <p>Card ID</p>
        <p>Element</p>
        <p>Skills</p>
        <p>Rarity</p>
        <p>Card Style</p>
        <p>Wear</p>
        <p>Image</p>
        <p>delete</p>
        <p>edit</p> 
    </div>
    {#each selected_cards as card, i}
        <div class="selected_player_row">
            {#if i === editing_card_index}
                <span>
                    <input bind:value={selected_cards[i].name}>
                </span>
                <p>{card.id}</p>
                <p><select bind:value={selected_cards[i].element}>
                    <option>Air</option>
                    <option>Water</option>
                    <option>Earth</option>
                    <option>Fire</option>
                </select></p>
                <span class="row">
                    <div class="stack">
                    Level
                            <p>hitpoints: <input type="number" bind:value={selected_cards[i].skills.hitpoints.level}></p>
                            <p>attack: <input type="number" bind:value={selected_cards[i].skills.attack.level}></p>
                            <p>strength: <input type="number" bind:value={selected_cards[i].skills.strength.level}></p>
                            <p>defense: <input type="number" bind:value={selected_cards[i].skills.defense.level}></p>
                        
                    </div>
                    <div class="stack">
                    Expereience
                            <p>hitpoints: <input type="number" bind:value={selected_cards[i].skills.hitpoints.experience}></p>
                            <p>attack: <input type="number" bind:value={selected_cards[i].skills.attack.experience}></p>
                            <p>strength: <input type="number" bind:value={selected_cards[i].skills.strength.experience}></p>
                            <p>defense: <input type="number" bind:value={selected_cards[i].skills.defense.experience}></p>
                    </div>

                </span>
                <p>{selected_cards[i].rarity}</p>
                <p>{card.card_style}</p>
                <p>{card.wear}</p>
            {:else}
                <p>{card.name}</p>
                <p>{card.id}</p>
                <p>{card.element}</p>
                <span class="row">
                    <div class="stack">
                        Level
                                <p>hitpoints: {card.skills.hitpoints.level}</p>
                                <p>attack: {card.skills.attack.level}</p>
                                <p>strength: {card.skills.strength.level}</p>
                                <p>defense: {card.skills.defense.level}</p>
                    </div>
                    <div class="stack">
                        Expereience
                            <p>hitpoints: {card.skills.hitpoints.experience}</p>
                            <p>attack: {card.skills.attack.experience}</p>
                            <p>strength: {card.skills.strength.experience}</p>
                            <p>defense: {card.skills.defense.experience}</p>
                    </div>
                </span>
                <p>{card.rarity}</p>
                <p>{card.card_style}</p>
                <p>{card.wear}</p>
            {/if}
            <label for={`selected_card_picture-${i}`}><img src={selected_files[i] ? URL.createObjectURL(selected_files[i][0]) :  getImageUrl(card.image)} alt=""></label>
            <input on:change={(event) => {
                const file = event?.currentTarget?.files; // Get the selected file
                const reader = new FileReader(); // Create a FileReader object
        
                // Set up the onload event handler
                reader.onload = function (event) {
                    const arrayBuffer = event?.target?.result; // Get the ArrayBuffer
        
                    // Create a Uint8Array from the ArrayBuffer 
                    if (arrayBuffer && typeof(arrayBuffer) != "string") {
                        const uint8Array = new Uint8Array(arrayBuffer);
    
                        selected_cards[i].image = Array.from(uint8Array);
                        selected_cards[i].imageUrl = getImageUrl(selected_cards[i].image);
                    }
                };
                if (file) {
                    reader.readAsArrayBuffer(file[0]);
                }
                }} style="display: none" id={`selected_card_picture-${i}`} type="file" accept=".jpg, .jpeg, .png" bind:files={selected_files[i]}>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
            <span class="icon_button" on:click={async () => {editing_card_index = null; selected_files[i] = null; selected_cards.splice(i, 1); selected_files.splice(i, 1); await delete_card(card.id);}}>
                    <FaTrash/>
            </span>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        {#if i === editing_card_index}
            <div>
                <span class="editing_buttons">
                    <span class="icon_button" on:click={async () => {editing_card_index = null; selected_files[i] = null; selected_cards.splice(i, 1); selected_files.splice(i, 1); await delete_card(card.id);}}>
                        <FaCheck/>
                    </span>
                    <span class="icon_button" on:click={(e) => {editing_card_index = null; e.stopPropagation();}}>
                        <FaTimes/>
                    </span>
                </span>
            </div>
        {:else}
            <span class="icon_button"  on:click={async () => { 
                editing_card_index = i; await edit_card(card.id, card); selected_cards[i] = await fetch_one_card(card.id); if (selected_files[i]){ selected_files[i] = null;}}}>
                <FaEdit/>
            </span>
        {/if}
        </div>
    {/each}
        <div class="selected_player_row">
            <span>
                <input type="text" bind:value={card_name}>
            </span>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
            <span class="icon_button stack"on:click={async () => {  if (selected_player) { await add_card(selected_player.id, card_name,  files_image ?? []);  }}}>
                <FaRegWindowRestore/>Add card
            </span>

            <span>
                <label for="card_picture">Add card picture <img src={getImageUrl(files_image)} alt=""/></label>
                <input style="display: none" id="card_picture" type="file" accept=".jpg, .jpeg, .png" bind:files={files} on:change={(event) => {
                    const file = event?.currentTarget?.files; // Get the selected file
                    const reader = new FileReader(); // Create a FileReader object
            
                    // Set up the onload event handler
                    reader.onload = function (event) {
                        const arrayBuffer = event?.target?.result; // Get the ArrayBuffer
            
                        // Create a Uint8Array from the ArrayBuffer
                        if (arrayBuffer && typeof(arrayBuffer) !== 'string') {
                            const uint8Array = new Uint8Array(arrayBuffer);
                
                            // Use the Uint8Array for further processing
                            // (e.g., send it to a server, process it with JavaScript, etc.)
                            files_image = Array.from(uint8Array);
                        }
                    };
            
                    // Read the selected file as an ArrayBuffer
                    if (file) {
                        reader.readAsArrayBuffer(file[0]);
                    }}}>
            </span>
        </div>
    {/if}
{/if}
</div>

<style>
.spacer {
    height: 50px;
}

.row {
    display: flex !important;
    gap: 2rem;

}
.stack {
    display: flex;
    justify-content: center;
    align-items: center;
    text-align: center;
    flex-direction: column;
    width: max-content !important;
}

.stack > div {
    display: flex;
    text-align: justify;
}
img {
    max-width: 100px;
}
.table-container {
    
    display: table;
    width: 100%;
    }
    .player_row {
        display: table-row;
    }

    .player_row > * { 
    vertical-align: middle;
    display: table-cell;
    padding: 5px;
    border: 1px solid black;
    }

    .selected_player_table {
        display: table;
        width: 100%;

    }
    .selected_player_row {
        display: table-row;
    }
    .selected_player_row  > * {
        vertical-align: middle;
        display:table-cell;
        padding: 10px;
        border: 1px solid black;
    }
    .selected_player_row .stack input {
        width: 2rem;
    }



    .icon_button {
        aspect-ratio: 1/1;
        height: 20px;
        max-width: 20px;
    }

    .icon_button:hover {
        color: whitesmoke;
    }

    .editing_buttons {
        height: 20px;
        display: flex;
        justify-content: space-around;
    }
</style> 
