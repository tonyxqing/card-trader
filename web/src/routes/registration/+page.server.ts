/** @type {import('./$types').Actions} */

export const actions = {
	default: async (event) => {
		const data = await event.request.formData();
		const payload: Record<string, File | string> = {};
		data.forEach((value, key) => {
			payload[key] = value;
		});
		payload['role'] = 'Basic';
		console.log(payload);
		const succ_registration = await fetch('http://localhost:5000/api/register-player', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(payload)
		});
		let registration_response;
		try {
			registration_response = await succ_registration.json();
		} catch {
			registration_response = await succ_registration.text();
			
		}

		console.log(registration_response);
	}
};
