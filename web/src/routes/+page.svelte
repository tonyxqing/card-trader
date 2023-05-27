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

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
{#if players.length > 0}
{#each players as player}
    <div class="player_row">
        <p>{player.name}</p>
        <p>{player.id}</p>
        <p>{player.date_created}</p>
        <p>{player.last_updated}</p>
        <button on:click={() => delete_player(player.id)}>delete</button>
        <button on:click={() => edit_player(player.id, {name: "Tony"})}>edit</button>

    </div>
{/each}
<p></p>
{/if}
<input bind:value={name}>
<button on:click={() => add_player(name)}>add player</button>

<style>
    .player_row {
        display: flex;
        gap: 10px;
    }
</style>
