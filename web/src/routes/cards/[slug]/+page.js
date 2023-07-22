/** @type {import('./$types').PageLoad} */

export async function load({ fetch, params }) {
    try {
        
        const req = await fetch(`https://localhost:5000/cards/${params.slug}`);
        const data = await req.text();
        if (req.ok) {
            console.log(data);
        }
        return {response: "hi"};
    } catch {
        return { response: "Hi"};
    }
}