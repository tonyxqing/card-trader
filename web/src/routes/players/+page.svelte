<script lang="ts">
    type Player = { 
        name: string, 
        id: string, 
        date_created: string, 
        last_updated: string 
    }

    async function get_players() {
        const req = await fetch("https://localhost:8080/players", {
            method: "GET",
        });
        const data = await req.json();
        players = data;
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
            body: JSON.stringify({ name })
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
    let name = "";
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
    <div class="player_row">    
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
                <button on:click={() => edit_player(player.id, player)}>edit</button>
            </span>
        </div>
    {/each}
    {/if}
</div>
<input bind:value={name}>
<button on:click={() => add_player(name)}>add player</button>

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
</style>
