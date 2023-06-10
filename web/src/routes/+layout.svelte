<script lang="ts">
	import { onMount } from "svelte";
       import Icon from "../lib/assets/favicon.png"
       import { tweened } from 'svelte/motion';
       let options = { duration: 10000, } // Specify the easing function
       let deg = tweened(0, options);
       onMount(async () => {
            const max = 360;
            const min = 0;
            let frame: number;
            let increasing = false;
            deg = tweened(0, options);
            async function animate() {
                if ($deg > max - 1 && increasing) {
                    await deg.set(min); // Set value1 to its maximum value
                    increasing = false;
                }

                if ($deg < min + 1 && !increasing) {
                    await deg.set(max); // Set value1 to its maximum value
                    increasing = true;
                }
                frame = requestAnimationFrame(animate);

            }
            await animate();
            return () => cancelAnimationFrame(frame);
       })


       let colors = ["#fff989", "#89d9ff"]
       $: degString = `${Math.floor($deg)}deg`;
       $: console.log(degString);
</script>


<nav
style="
    --deg: {degString};
    --gradient-1: {colors[0]};
    --gradient-2: {colors[1]};
">
    <img class="nav_icon" src={Icon} alt="favicon"/>
    <a href="/">Home</a>
    <a href="/players">Players</a>
    <a href="/cards">Cards</a>
</nav>

<slot></slot>

<style>
    :global(body) {
        background: linear-gradient(to bottom, #4e54c8, #8f94fb);
        background-attachment: fixed;
        margin: 0;
    }
    nav {
        display: flex;
        background-color: #a6bdfc;
        align-items: center;
        height: 84px;
		border: 2px solid black;
        background: linear-gradient(var(--deg), var(--gradient-1), var(--gradient-2));
    }

    nav a {
        padding: 2rem 5rem;
        text-decoration: none;
        font-family: 'Lucida Sans', 'Lucida Sans Regular', 'Lucida Grande', 'Lucida Sans Unicode', Geneva, Verdana, sans-serif;
    }

    nav a:hover {
        color: #8f94fb;
    }
    .nav_icon {
        padding: 0 2rem;
        height: 80%;
        max-width: auto;
    }


</style>