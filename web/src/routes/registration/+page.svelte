<div class="container">
	<form method="POST">
		<label>
			Username
			<input name="username" type="username" required />
		</label>
		<label>
			Phone Number
			<input name="phone" type="tel" maxlength="10" pattern={String.raw`[0-9]{10}`} required />
		</label>
		<label>
			Passcode
			<input name="passcode" type="password" maxlength="8" required />
		</label>
		<button>Create Account</button>
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

	input {
		height: 32px;
	}
</style>
