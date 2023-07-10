<script>
	let passcode = '';
	let retypedPasscode = '';
	let email = '';
	let username = '';
</script>

<div class="container">
	<form method="POST">
		<label>
			Username
			<input
				name="username"
				type="username"
				bind:value={username}
				minlength="3"
				maxlength="16"
				required
			/>
		</label>
		<label>
			Email
			<input name="email" type="email" bind:value={email} required />
		</label>
		<label>
			Passcode
			<input
				name="passcode"
				type="password"
				bind:value={passcode}
				minlength="8"
				maxlength="128"
				required
			/>
		</label>
		<label>
			Re-Type Passcode
			<input
				name="passcode"
				type="password"
				minlength="8"
				bind:value={retypedPasscode}
				maxlength="128"
				required
			/>
		</label>
		<button disabled={ passcode && passcode !== retypedPasscode || email.length < 3 || username.length < 3 }>Create Account</button>
	</form>
</div>

<style>
	.container {
		display: flex;
		width: 100%;
		justify-content: center;
	}
	form {
		top: 120px;
		position: absolute;
		background-color: whitesmoke;
		padding: 20px;
		border-radius: 10px;
		display: flex;
		gap: 20px;
		width: 320px;
		align-self: center;
		flex-direction: column;
	}
	form > label {
		display: flex;
		flex-direction: column;
		font-size: 20px;
		font-family: sans-serif;
		color: #aaaaaa;
		gap: 10px;
	}

	button {
		height: 48px;
		font-size: 16px;
		color: #000;
		border-color: transparent;
		border-radius: 4px;
		background-color: #42f692;
	}
	button:hover {
		background-color: #44ffa9;
	}
	button:disabled,
	button[disabled] {
		border: 1px solid #999999;
		background-color: #cccccc;
		color: #666666;
	}
	input {
		height: 32px;
	}
</style>
