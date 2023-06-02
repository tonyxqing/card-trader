/** @type {import('./$types').PageLoad} */

export async function load({ fetch, params }) {
    try {
        const req = await fetch(`https://localhost:8080/cards/${params.slug}`);
        const data = await req.text();
        if (req.ok) {
            console.log(data);
        }
        return {response: "hi"};
    } catch {
        return { response: "Hi"};
    }
}