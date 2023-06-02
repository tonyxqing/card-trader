<script lang="ts">
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
        skills: Skills,
        owner_id: string,
        imageUrl: string,
    }
    
    type Player = { 
        name: string, 
        id: string, 
        date_created: string, 
        last_updated: string,
        cards: Card[] 
    }

    async function get_players() {
        const req = await fetch("https://localhost:8080/players", {
            method: "GET",
        });
        const data = await req.json();
        players = data;
    }

    async function get_player(player_id: string) {
        const req = await fetch(`https://localhost:8080/players/${player_id}`, {
            method: "GET",
        });
        const data = await req.json();
        return data;
    }

    async function add_card(player_id: string, name: string, files: any) {
        const req = await fetch(`https://localhost:8080/players/${player_id}/cards`, {
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
        const req = await fetch(`https://localhost:8080/cards/${card_id}`, {
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
        const req = await fetch(`https://localhost:8080/players/${id}/cards`, {
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
        const req = await fetch(`https://localhost:8080/cards/${card_id}`, {
            method: "DELETE",
        })
    }

    async function edit_card(card_id: string, card: Card) {
        const req = await fetch(`https://localhost:8080/cards/${card_id}`, {
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
        const req = await fetch("https://localhost:8080/players", {
            method: "POST",
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ name })
        });
        get_players()
    }
    async function delete_player(id: string) {
        const req = await fetch(`https://localhost:8080/players/${id}`, {
            method: "DELETE",
        });
        get_players()
    }

    async function edit_player(id: string, obj: Partial<Player>) {
        const response = await fetch(`https://localhost:8080/players/${id}`, {
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
    $: console.log(selected_files);
</script>

<h1>Welcome to Player Admin Portal</h1>
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
    {#each players as player}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="player_row" on:click={async () => {selected_player = await get_player(player.id); if (selected_player) {await fetch_player_cards(selected_player.id)}}}>    
            <span>
                <input bind:value={player.name}>
            </span>
            <p>{player.id}</p>
            <p>{player.date_created}</p>
            <p>{player.last_updated}</p>
            <span>
                <button on:click={async (e) => {if (player.id === selected_player?.id) {await delete_player(player.id); selected_cards = []; selected_player = null} else {await delete_player(player.id)} e.stopPropagation();}}>delete</button>
            </span>
            <span>
                <button on:click={(e) => {edit_player(player.id, player); e.stopPropagation();}}>update</button>
            </span>
        </div>
    {/each}
    {/if}
    <div class="player_row">
        <span>
            <input type="text" bind:value={player_name}>
        </span>
        <span>
            <button on:click={() => add_player(player_name)}>add player</button>
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
        <p>Image</p>
    </div>
    {#each selected_cards as card, i}
        <div class="selected_player_row">
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
            <span class="stack">
                Level
                <div>
                        <p>hitpoints: <input type="number" bind:value={selected_cards[i].skills.hitpoints.level}></p>
                        <p>attack: <input type="number" bind:value={selected_cards[i].skills.attack.level}></p>
                        <p>strength: <input type="number" bind:value={selected_cards[i].skills.strength.level}></p>
                        <p>defense: <input type="number" bind:value={selected_cards[i].skills.defense.level}></p>
                    
                </div>
                Expereience
                <div>
                        <p>hitpoints: <input type="number" bind:value={selected_cards[i].skills.hitpoints.experience}></p>
                        <p>attack: <input type="number" bind:value={selected_cards[i].skills.attack.experience}></p>
                        <p>strength: <input type="number" bind:value={selected_cards[i].skills.strength.experience}></p>
                        <p>defense: <input type="number" bind:value={selected_cards[i].skills.defense.experience}></p>
                </div>

            </span>
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
            
            <span>
                <button on:click={async () => {selected_files[i] = null; selected_cards.splice(i, 1); selected_files.splice(i, 1); await delete_card(card.id);}}>delete</button>
            </span>
            <span>
                <button on:click={async () => { 
                     await edit_card(card.id, card); selected_cards[i] = await fetch_one_card(card.id); if (selected_files[i]){ selected_files[i] = null;}}}>update</button>
            </span>
        </div>
    {/each}
        <span>
            <input type="text" bind:value={card_name}>
            <button on:click={async () => {  if (selected_player) { await add_card(selected_player.id, card_name,  files_image ?? []);  }}}>add card</button>
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
                }
                }
                }>
        </span>
    {/if}
{/if}
</div>

<style>
.spacer {
    height: 50px;
}
.stack {
    display: flex;
    flex-direction: column;
    width: 100%;
}

.stack > div {
    display: flex;
}
img {
    max-width: 250px;
    max-height:350px;
}
.table-container {
    
    display: table;
    width: 100%;
    }
    .player_row {
        display: table-row;
    }

    .player_row > * { 
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
        vertical-align: top;
        display:table-cell;
        padding: 10px;
        border: 1px solid black;
    }
</style>
