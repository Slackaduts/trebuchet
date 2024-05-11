<script>
    import { appWindow } from '@tauri-apps/api/window'
    import { Button } from "$lib/components/ui/button";
    import { X } from 'lucide-svelte';

    export let size = 24;

    async function close() {
        await appWindow.close();
    }
</script>
  

<Button variant="ghost" on:click="{close}" size = "icon">
    <X size = {size}/>
</Button>
