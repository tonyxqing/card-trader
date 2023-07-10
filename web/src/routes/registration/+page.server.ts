/** @type {import('./$types').Actions} */

export const actions = {
	default: async (event) => {
		const data = await event.request.formData();
		const payload: Record<string, File | string> = {};
		data.forEach((value, key) => {
			payload[key] = value;
		});
		payload['role'] = 'Basic';
		const succ_registration = await fetch('http://localhost:8080/api/register-player', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(payload)
		});
		const registration_response = await succ_registration.json();
		console.log(registration_response);
	}
};
