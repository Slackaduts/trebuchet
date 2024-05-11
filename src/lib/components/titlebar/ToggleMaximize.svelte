<script>
    import { appWindow } from '@tauri-apps/api/window'
    import { Button } from "$lib/components/ui/button";
    import { Maximize, Minimize } from 'lucide-svelte';

    let show = false;
    export let size = 24;

    async function toggleMaximize() {
        await appWindow.toggleMaximize();
        show = await appWindow.isMaximized();
    } 
</script>
  

<Button variant = "ghost" on:click="{toggleMaximize}" size = "icon">
    { #if !(show) }
        <Maximize size = {size}/>
    {:else}
        <Minimize size = {size}/>
    {/if}
</Button>