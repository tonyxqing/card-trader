<div class="container">
	<form method="POST">
		<label>
			Username
			<input name="username" type="username" required />
		</label>
		<label>
			Passcode
			<input name="passcode" type="password" required />
		</label>
		<a href="/registration">register a new account</a>
		<button>Log in</button>
	</form>
</div>

<style>
	.container {
		display: flex;
		width: 100%;
		justify-content: center;
		align-items: center;
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
		justify-content: space-between;
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
	input {
		height: 32px;
	}
</style>
