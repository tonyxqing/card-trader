/** @type {import('./$types').Actions} */
export const actions = {
    default: async ({request}) => {
        // TODO log the user in
        const data = await request.formData()
        console.log("Trying to Log in", data);
        
    }
};