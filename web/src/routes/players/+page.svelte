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
        players = data;
    }

    async function get_player(id: string) {
        const req = await fetch(`https://localhost:8080/players/${id}`, {
            method: "GET",
        });
        const data = await req.json();
        return data;
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
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ name: player_name })
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
    let players: Player[] = [];
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
    <div class="player_row" on:click={async () => {selected_player = await get_player(player.id)}}>    
            <span>
                <input bind:value={player.name}>
            </span>
            <p>{player.id}</p>
            <p>{player.date_created}</p>
            <p>{player.last_updated}</p>
            <span>
                <button on:click={() => delete_player(player.id)}>delete</button>
            </span>
            <span>
                <button on:click={() => edit_player(player.id, player)}>update</button>
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
    <div class="selected_player_row">
        {#each selected_player.cards as card}
            <p>{card.name}</p>
        {/each}
        <span>
            <input type="text" bind:value={card_name}>
            <button on:click={()=>{}}>add card</button>
        </span>
        <span>
            <label for="card_picture">Add card picture</label>
            <input style="opacity: 0" id="card_picture" type="file" accept=".jpg, .jpeg, .png" >
        </span>
    </div>

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
