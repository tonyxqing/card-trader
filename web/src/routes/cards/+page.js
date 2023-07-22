/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
    try {

        const req = await fetch("https://localhost:5000/cards", {
            method: "GET"
        })
    
        const data = await req.text();
        if (req.ok) {
            console.log(data);
        }
        return { response: "hi"};
    } catch {
        return { response: "hi" };
    }
}