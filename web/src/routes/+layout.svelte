<script lang="ts">
	import { onMount } from 'svelte';
	import Icon from '../lib/assets/favicon.png';
	import { tweened } from 'svelte/motion';
	import { expoIn, quadIn, quadInOut } from 'svelte/easing';
	import { writable } from 'svelte/store';
	export const LoginSession = false;

	let options = { duration: 5000 }; // Specify the easing function
	let deg = tweened(0, options);
	onMount(async () => {
		const max = 360;
		const min = 0;
		let frame: number;
		let increasing = false;
		async function animate() {
			deg = tweened(0, options);
			await deg.set(max);
			frame = requestAnimationFrame(animate);
		}
		await animate();
		return () => cancelAnimationFrame(frame);
	});

	let colors = ['#fff989', '#89d9ff'];
	$: degString = `${Math.floor($deg)}deg`;
	$: console.log(degString);
</script>

<nav
	style="
    --deg: {degString};
    --gradient-1: {colors[0]};
    --gradient-2: {colors[1]};
"
>
	<img class="nav_icon" src={Icon} alt="favicon" />
	<a href="/">Home</a>
	<a href="/players">Players</a>
	<a href="/cards">Cards</a>
	<a href="/battle">Battle</a>
	{#if LoginSession}
		<!-- content here -->
		<p>Welcome</p>
		// display username
	{:else}
		<!-- else content here -->Login
		<form method="POST" action="/login">
			<label>
				Username
				<input name="username" type="username" required />
			</label>
			<label>
				Password
				<input name="password" type="password" required />
			</label>
			<button>submit</button>
		</form>
	{/if}
</nav>

<slot />

<style>
	:global(body) {
		background: linear-gradient(to bottom, #4e54c8, #8f94fb);
		background-attachment: fixed;
		margin: 0;
		font-family: 'Comic Sans MS';
	}
	nav {
		display: flex;
		background-color: #a6bdfc;
		align-items: center;
		height: 84px;
		width: 100%;
		min-width: fit-content;
		background: center / cover no-repeat fixed
			linear-gradient(var(--deg), var(--gradient-1), var(--gradient-2));
	}

	nav a {
		flex: 1 1 100%;
		padding: 2rem 5rem;
		text-decoration: none;
		font-family: 'Lucida Sans', 'Lucida Sans Regular', 'Lucida Grande', 'Lucida Sans Unicode',
			Geneva, Verdana, sans-serif;
	}

	nav a:hover {
		color: #8f94fb;
	}
	.nav_icon {
		padding: 0 2rem;
		height: 80%;
		max-width: auto;
	}
</style>
