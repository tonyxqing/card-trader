<script lang="ts">
	import { onMount } from "svelte";

    let socket: WebSocket;
    onMount(() => {
        console.log("Attempting to connect websocket")
        socket = new WebSocket("ws://localhost:8080/ws");
        // Connection opened
        socket.addEventListener("open", (event) => {
            socket.send("Hello Server!");
        });
        
        // Listen for messages
        socket.addEventListener("message", (event) => {
            console.log("Message from server ", event.data);
        });
    })
</script>

<div>
    Welcome
</div>