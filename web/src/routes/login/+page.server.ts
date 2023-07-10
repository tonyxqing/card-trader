/** @type {import('./$types').Actions} */
export const actions = {
	default: async ({ request, fetch }) => {
		const data = await request.formData();
		const payload: Record<string, File | string> = {};
		data.forEach((value, key) => {
			payload[key] = value;
		});
		const succ_login = await fetch('http://localhost:8080/api/authenticate-player', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(payload)
		});
		const login_response = await succ_login.text();
		console.log(login_response);
	}
};
