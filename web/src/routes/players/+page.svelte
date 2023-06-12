<script lang="ts">
	import { linear } from 'svelte/easing';
	import FaTrash from 'svelte-icons/fa/FaTrash.svelte';
	import FaEdit from 'svelte-icons/fa/FaEdit.svelte';
	import FaCheck from 'svelte-icons/fa/FaCheck.svelte';
	import FaTimes from 'svelte-icons/fa/FaTimes.svelte';
	import FaUserPlus from 'svelte-icons/fa/FaUserPlus.svelte';
	import FaRegWindowRestore from 'svelte-icons/fa/FaRegWindowRestore.svelte';
	import GiMoneyStack from 'svelte-icons/gi/GiMoneyStack.svelte';
	import GiCoins from 'svelte-icons/gi/GiCoins.svelte';
	interface Skill {
		level: number;
		experience: number;
	}
	interface Skills {
		attack: Skill;
		defense: Skill;
		strength: Skill;
		hitpoints: Skill;
	}

	type Card = {
		id: string;
		name: string;
		element: string;
		image: number[];
		imageUrl: string;
		skills: Skills;
		rarity: string;
		card_style: string;
		wear: number;
		owner_id: string;
	};

	type Player = {
		name: string;
		id: string;
		date_created: string;
		last_updated: string;
		social_credit: number;
		dink_coin: number;
	};

	async function get_players() {
		const req = await fetch('http://localhost:8080/players', {
			method: 'GET'
		});
		const data = await req.json();
		players = data;
	}

	async function get_player(player_id: string) {
		const req = await fetch(`http://localhost:8080/players/${player_id}`, {
			method: 'GET'
		});
		const data = await req.json();
		return data;
	}

	async function add_card(player_id: string, name: string, files: any) {
		const req = await fetch(`http://localhost:8080/players/${player_id}/cards`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ name, image: files })
		});
		const data = await req.json();
		selected_cards = [...selected_cards, data];
		console.log(selected_cards, data);
		return data;
	}
	async function fetch_one_card(card_id: string) {
		const req = await fetch(`http://localhost:8080/cards/${card_id}`, {
			method: 'GET'
		});

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
			method: 'GET'
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
			method: 'DELETE'
		});
	}

	async function edit_card(card_id: string, card: Card) {
		const req = await fetch(`http://localhost:8080/cards/${card_id}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				name: card.name,
				image: card.image,
				skills: card.skills,
				element: card.element,
				owner_id: card.owner_id
			})
		});
	}

	async function add_player(name: string) {
		const req = await fetch('http://localhost:8080/players', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ name })
		});
		get_players();
	}
	async function delete_player(id: string) {
		const req = await fetch(`http://localhost:8080/players/${id}`, {
			method: 'DELETE'
		});
		get_players();
	}

	async function edit_player(obj: Partial<Player>) {
		const response = await fetch(`http://localhost:8080/players/${obj.id}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(obj)
		});
		get_players();
	}

	get_players();
	let selected_files: any = [];
	let player_name = '';
	let card_name = '';
	let selected_player: Player | null;
	let selected_cards: Card[];
	let players: Player[] = [];
	let files: any;
	let files_image: number[];
	let editing_player_index: number | null;
	let editing_card_index: number | null;

	$: console.log(selected_cards);
</script>

<div class="spacer" />
<div style="display: flex; align-items: center; gap: 1rem;">
	<h3>Player List</h3>
	<p>New Name:</p>
	<span>
		<input type="text" bind:value={player_name} />
	</span>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		style="display: flex; align-items: center; gap: 1rem; "
		class="icon_button"
		on:click={() => add_player(player_name)}
	>
		<span class="icon_button">
			<FaUserPlus />
		</span>
		<p>Add player</p>
	</div>
</div>
<div class="player_container">
	{#if players.length > 0}
		{#each players as player, i}
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<div
				class="player_card"
				on:click={async () => {
					selected_player = await get_player(player.id);
					if (selected_player) {
						await fetch_player_cards(selected_player.id);
					}
				}}
			>
				{#if i === editing_player_index}
					<div>
						<div
							style="display: flex; justify-content: space-between; align-items: center; gap: 1rem"
						>
							<h3>
								<input bind:value={player.name} />
							</h3>
						</div>
					</div>
					<div class="row">
						<div style="display: flex; align-items: center; gap: 1rem;">
							<span class="icon_button">
								<GiCoins />
							</span>
							<input type="number" bind:value={player.dink_coin} />
						</div>
						<div style="display: flex; align-items:center; gap: 1rem;">
							<span class="icon_button">
								<GiMoneyStack />
							</span>
							<input type="number" bind:value={player.social_credit} />
						</div>
					</div>
					<div>
						<span class="editing_buttons">
							<span
								class="icon_button"
								on:click={(e) => {
									edit_player(player);
									editing_player_index = null;
									e.stopPropagation();
								}}
							>
								<FaCheck />
							</span>
							<span
								class="icon_button"
								on:click={(e) => {
									editing_player_index = null;
									e.stopPropagation();
								}}
							>
								<FaTimes />
							</span>
						</span>
					</div>
				{:else}
					<div
						style="display: flex; justify-content: space-between; align-items: center; gap: 1rem; padding: 0 1rem;"
					>
						<h3>{player.name}</h3>
						<div style="display: flex; gap: 1rem;">
							<span
								class="icon_button"
								on:click={async (e) => {
									if (player.id === selected_player?.id) {
										await delete_player(player.id);
										selected_cards = [];
										selected_player = null;
									} else {
										await delete_player(player.id);
									}
									e.stopPropagation();
								}}
							>
								<FaTrash />
							</span>
							<span
								class="icon_button"
								on:click={(e) => {
									editing_player_index = i;
									e.stopPropagation();
								}}
							>
								<FaEdit />
							</span>
						</div>
					</div>
					<div class="row">
						<div style="display: flex; align-items: center; gap: 1rem; padding: 0 1rem;">
							<span class="icon_button">
								<GiCoins />
							</span>
							<p style="margin: 0;">
								{player.dink_coin}
							</p>
						</div>
						<div style="display: flex; align-items:center; gap: 1rem; padding: 0 1rem;">
							<span class="icon_button">
								<GiMoneyStack />
							</span>
							<p style="margin: 0;">{player.social_credit}</p>
						</div>
					</div>
				{/if}
				<div class="card_label">
					<p>{player.id}</p>
				</div>
			</div>
		{/each}
	{/if}
</div>
<div class="spacer" />
{#if selected_player}
	<div
		style="display: flex; flex-wrap: wrap;justify-content:center;align-items: center; gap: 1rem;"
	>
		<h3>Selected Player Cards</h3>
		<p>{selected_player.name}</p>
		<p>{selected_player.id}</p>
		<div class="spacer" />
		<div style="display: flex; align-items: center; gap: 1rem; ">
			<span>
				<input type="text" bind:value={card_name} />
			</span>

			<span>
				<label for="card_picture"
					>Add card picture <img src={getImageUrl(files_image)} alt="" /></label
				>
				<input
					style="display: none"
					id="card_picture"
					type="file"
					accept=".jpg, .jpeg, .png"
					bind:files
					on:change={(event) => {
						const file = event?.currentTarget?.files; // Get the selected file
						const reader = new FileReader(); // Create a FileReader object

						// Set up the onload event handler
						reader.onload = function (event) {
							const arrayBuffer = event?.target?.result; // Get the ArrayBuffer

							// Create a Uint8Array from the ArrayBuffer
							if (arrayBuffer && typeof arrayBuffer !== 'string') {
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
					}}
				/>
			</span>
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<div
				style="display: flex; align-items: center; gap: 1rem;"
				class="icon_button row"
				on:click={async () => {
					if (selected_player) {
						await add_card(selected_player.id, card_name, files_image ?? []);
					}
				}}
			>
				<span class="icon_button">
					<FaRegWindowRestore />
				</span>
				<p>Add card</p>
			</div>
		</div>
	</div>
	<div class="spacer" />
	<div class="selected_player_table">
		{#if selected_cards}
			{#each selected_cards as card, i}
				<div class="selected_player_card">
					{#if i === editing_card_index}
						<h3>
							<input bind:value={selected_cards[i].name} />
						</h3>
						<div class="row">
							<label for={`selected_card_picture-${i}`}>
								<img
									src={selected_files[i]
										? URL.createObjectURL(selected_files[i][0])
										: getImageUrl(card.image)}
									alt=""
								/>
							</label>
							<input
								on:change={(event) => {
									const file = event?.currentTarget?.files; // Get the selected file
									const reader = new FileReader(); // Create a FileReader object

									// Set up the onload event handler
									reader.onload = function (event) {
										const arrayBuffer = event?.target?.result; // Get the ArrayBuffer

										// Create a Uint8Array from the ArrayBuffer
										if (arrayBuffer && typeof arrayBuffer != 'string') {
											const uint8Array = new Uint8Array(arrayBuffer);

											selected_cards[i].image = Array.from(uint8Array);
											selected_cards[i].imageUrl = getImageUrl(selected_cards[i].image);
										}
									};
									if (file) {
										reader.readAsArrayBuffer(file[0]);
									}
								}}
								style="display: none"
								id={`selected_card_picture-${i}`}
								type="file"
								accept=".jpg, .jpeg, .png"
								bind:files={selected_files[i]}
							/>
							<span class="row">
								<div class="stack">
									Level
									<p>
										hitpoints: <input
											type="number"
											bind:value={selected_cards[i].skills.hitpoints.level}
										/>
									</p>
									<p>
										attack: <input
											type="number"
											bind:value={selected_cards[i].skills.attack.level}
										/>
									</p>
									<p>
										strength: <input
											type="number"
											bind:value={selected_cards[i].skills.strength.level}
										/>
									</p>
									<p>
										defense: <input
											type="number"
											bind:value={selected_cards[i].skills.defense.level}
										/>
									</p>
								</div>
								<div class="stack">
									Expereience
									<p>
										hitpoints: <input
											type="number"
											bind:value={selected_cards[i].skills.hitpoints.experience}
										/>
									</p>
									<p>
										attack: <input
											type="number"
											bind:value={selected_cards[i].skills.attack.experience}
										/>
									</p>
									<p>
										strength: <input
											type="number"
											bind:value={selected_cards[i].skills.strength.experience}
										/>
									</p>
									<p>
										defense: <input
											type="number"
											bind:value={selected_cards[i].skills.defense.experience}
										/>
									</p>
								</div>
							</span>
						</div>
						<div class="row">
							<p>
								<select bind:value={selected_cards[i].element}>
									<option>Air</option>
									<option>Water</option>
									<option>Earth</option>
									<option>Fire</option>
								</select>
							</p>
							<p>{selected_cards[i].rarity}</p>
							<p>{card.card_style}</p>
						</div>
					{:else}
						<div class="row">
							<h3>{card.name}</h3>
							<div style="display: flex; gap: 1rem;">
								<!-- svelte-ignore a11y-click-events-have-key-events -->
								<span
									class="icon_button"
									on:click={async () => {
										editing_card_index = null;
										selected_cards = selected_cards.splice(i, 1);
										selected_files = selected_files.splice(i, 1);
										await delete_card(card.id);
									}}
								>
									<FaTrash />
								</span>
								<!-- svelte-ignore a11y-click-events-have-key-events -->
								<span
									class="icon_button"
									on:click={async () => {
										editing_card_index = i;
									}}
								>
									<FaEdit />
								</span>
							</div>
						</div>
						<div class="row">
							<img src={getImageUrl(card.image)} alt="" />
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
						</div>
						<div class="row">
							<p>{card.element}</p>
							<p>{card.rarity}</p>
							<p>{card.card_style}</p>
						</div>
					{/if}

					<div class="row">
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						{#if i === editing_card_index}
							<span class="editing_buttons row">
								<span
									class="icon_button"
									on:click={async () => {
										editing_card_index = null;
										selected_files[i] = null;
										await edit_card(card.id, card);
										selected_cards[i] = await fetch_one_card(card.id);
									}}
								>
									<FaCheck />
								</span>
								<span
									class="icon_button"
									on:click={(e) => {
										editing_card_index = null;
										e.stopPropagation();
									}}
								>
									<FaTimes />
								</span>
							</span>
						{/if}
					</div>
					<div class="row">
						<div class="card_label">
							<p>{card.id}</p>
						</div>
						<div class="card_label">
							<p>{card.wear}</p>
						</div>
					</div>
				</div>
			{/each}
		{/if}
	</div>
{/if}
<div class="spacer" />

<style>
	h3 {
		padding-left: 1rem;
	}
	.spacer {
		height: 50px;
	}

	.row {
		display: flex !important;
		gap: 1rem;
		align-items: center;
		padding: 0 1rem;
		width: -webkit-fill-available;
		width: -moz-available;
		justify-content: space-between;
	}
	.stack {
		display: flex;
		text-align: left;
		flex-direction: column;
		width: max-content !important;
	}

	.stack > div {
		display: flex;
		text-align: justify;
	}

	.player_container {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
		margin: 1rem;
	}

	.player_card:not(:first-child):not(:last-child) > * {
	}
	.player_card {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		border: 1px solid black;
		border-radius: 1rem;
		flex: 0 1 30%;
		background: center / contain fixed no-repeat linear-gradient(whitesmoke, #8f94fb);
	}
	.card_label {
		line-height: 1rem;
	}

	.card_label b {
		font-size: 12px;
	}
	.card_label p {
		color: rgb(132, 132, 132);
		font-size: 12px;
		padding-left: 1rem;
		text-wrap: nowrap;
	}
	.player_card > * {
		vertical-align: middle;
		padding: 5px;
		/* border: 1px solid black; */
	}

	.selected_player_table {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}
	.selected_player_card {
		display: flex;
		align-items: center;
		flex-direction: column;
		justify-content: space-between;
		border: 1px solid black;
		border-radius: 1rem;
		background: center / contain fixed no-repeat linear-gradient(whitesmoke, #8f94fb);
		flex: 0 1 30%;
	}

	.selected_player_card .stack input {
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
