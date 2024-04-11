<script lang="ts">
	import "$lib/../app.pcss";
    // import '$lib/../app.postcss';

    import * as Dialog from "$lib/components/ui/dialog";
    import { Button } from "$lib/components/ui/button";

    import {PackageCheck} from 'lucide-svelte';

    export let titlebar_icon_size = 24;
</script>


<Dialog.Root>
    <Dialog.Trigger>
        <dev class = "flex items-center">
            <Button variant = "outline" size = "icon">
                <PackageCheck size = {titlebar_icon_size}/>

            </Button>
        </dev>
    </Dialog.Trigger>
    <Dialog.Content>
        <Dialog.Header>
            <Dialog.Title></Dialog.Title>
            <Dialog.Description>
                This action cannot be undone. This will permanently delete your account
                and remove your data from our servers.
            </Dialog.Description>
        </Dialog.Header>
    </Dialog.Content>
</Dialog.Root>