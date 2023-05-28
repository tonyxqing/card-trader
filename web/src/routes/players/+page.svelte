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
        image: string,
        skills: Skills,
        owner_id: string
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
        console.log(data);
        players = data;
    }

    async function get_player(player_id: string) {
        const req = await fetch(`https://localhost:8080/players/${player_id}`, {
            method: "GET",
        });
        const data = await req.json();
        return data;
    }

    async function add_card(player_id: string, files: any) {
        const req = await fetch(`https://localhost:8080/players/${player_id}/cards`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({owner_id: player_id, files}),
        })
        const data = await req.json();
        selected_cards = await fetch_player_cards(player_id)

    }

    async function fetch_player_cards(id: string) {
        const req = await fetch(`https://localhost:8080/players/${id}/cards`, {
            method: "GET",
        });
        const data = await req.json();

        for (let d in data) {

            const byteArray = new Uint8Array(data[d].image);
    
            // Convert the byte array to a Blob
            const blob = new Blob([byteArray], { type: 'image/jpeg' });
    
            // Create an object URL for the Blob
            const imageUrl = URL.createObjectURL(blob);
            data[d].image = imageUrl;
            console.log(data[d]);
        }
        return data;
    }
    async function delete_card(id: string) {
        const req = fetch(`https://localhost:8080/cards/{$id}`, {
            method: "DELETE",
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
    let player_name = "";
    let card_name = "";
    let selected_player: Player;
    let selected_cards: Card[];
    let players: Player[] = [];
    let files: any;
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
    <div class="player_row" on:click={async () => {selected_player = await get_player(player.id); selected_cards = await fetch_player_cards(selected_player.id)}}>    
            <span>
                <input bind:value={player.name}>
            </span>
            <p>{player.id}</p>
            <p>{player.date_created}</p>
            <p>{player.last_updated}</p>
        </div>
        <span>
            <button on:click={() => delete_player(player.id)}>delete</button>
        </span>
        <span>
            <button on:click={() => edit_player(player.id, player)}>update</button>
        </span>
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
        <p>Image</p>
    </div>
    {#each selected_cards as card}
        <div class="selected_player_row">
            <p>{card.name}</p>
            <p>{card.id}</p>
            <p>{card.element}</p>
            <p><img src={card.image} alt=""></p>
            <span>
                <button on:click={() => delete_card(card.id)}>delete</button>
            </span>
            <span>
                <!-- <button on:click={() => edit_card(card.id, card)}>update</button> -->
            </span>
        </div>
    {/each}
        <span>
            <input type="text" bind:value={card_name}>
            <button on:click={async ()=>{await add_card(selected_player.id, files); }}>add card</button>
        </span>
        <span>
            <label for="card_picture">Add card picture</label>
            <input style="opacity: 0" id="card_picture" type="file" accept=".jpg, .jpeg, .png" bind:files={files}>
        </span>
    {/if}
{/if}
</div>
<style>

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
    .selected_player_row * {
        display:table-cell;
        padding: 10px;
        border: 1px solid black;
    }
</style>
