<script>
    import { appWindow } from '@tauri-apps/api/window'
    import { Button } from "$lib/components/ui/button";
    import { Minus } from 'lucide-svelte';

    export let size = 24;

    async function minimize() {
        await appWindow.minimize();
    } 
</script>


<Button variant="ghost" on:click="{minimize}" size = "icon">
    <Minus size = {size}/>
</Button>
