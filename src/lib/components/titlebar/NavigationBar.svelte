<script lang="ts">
    // import * as Sheet from "$lib/components/ui/sheet";
    import { Button, buttonVariants } from "$lib/components/ui/button";
    import {Separator} from "$lib/components/ui/separator";
    import * as HoverCard from "$lib/components/ui/hover-card";

    import {Bolt, Handshake, Home, Terminal} from 'lucide-svelte';

    export let titlebar_icon_size = 24;
</script>
 

<div data-tauri-drag-region class = "flex flex-row w-full justify-start items-center space-x-1">
    <HoverCard.Root>
        <HoverCard.Trigger>
            <Button variant="outline" size="icon" href = "/settings">
                <Bolt size={titlebar_icon_size}/>
            </Button>
        </HoverCard.Trigger>
        <HoverCard.Content>
            <h1 class = "text-lg font-semibold text-foreground">Settings</h1>
            <Separator/>
            Configure Trebuchet in a few easy clicks.
        </HoverCard.Content>
    </HoverCard.Root>

    <HoverCard.Root>
        <HoverCard.Trigger>
            <Button variant="outline" size="icon" href = "/accounts">
                <Handshake size={titlebar_icon_size} />
            </Button>
        </HoverCard.Trigger>
        <HoverCard.Content>
            <h1 class = "text-lg font-semibold text-foreground">Accounts</h1>
            <Separator/>
            Add, remove, and managed your Wizard101/Pirate101 accounts.
        </HoverCard.Content>
    </HoverCard.Root>

    <HoverCard.Root>
        <HoverCard.Trigger>
            <Button variant="outline" size="icon" href = "/">
                <Home size={titlebar_icon_size} />
            </Button>
        </HoverCard.Trigger>
        <HoverCard.Content>
            <h1 class = "text-lg font-semibold text-foreground">Home</h1>
            <Separator/>
            Launch, update, and manage Wizard101 or Pirate101 with any number of accounts.
        </HoverCard.Content>
    </HoverCard.Root>

    <HoverCard.Root>
        <HoverCard.Trigger>
            <Button variant="outline" size="icon" href = "/logs">
                <Terminal size={titlebar_icon_size} />
            </Button>
        </HoverCard.Trigger>
        <HoverCard.Content>
            <h1 class = "text-lg font-semibold text-foreground">Logs</h1>
            <Separator/>
            See live debug information for Trebuchet.
        </HoverCard.Content>
    </HoverCard.Root>
</div>


<!-- <Sheet.Root>
    <Sheet.Trigger>
        <Button variant="outline" size="icon">
            <Bolt size={titlebar_icon_size} />
        </Button>
    </Sheet.Trigger>
    <Sheet.Content side = "left">
        <div class = "flex flex-col w-full items-center justify-center">
            <div class = "rounded-md inline-flex justify-between w-full m-1 border border-input bg-background">
                <div class = "flex flex-row w-full justify-between items-center">
                    <span class = "text-lg font-semibold text-foreground">Navigation</span>
                    <Sheet.Close class = "flex justify-end">
                        <Button variant="ghost" size="icon">
                            <X size={titlebar_icon_size} class = "" />
                        </Button>
                    </Sheet.Close>
                </div>
            </div>

            <Separator class = "m-1"/>

            <div class = "m-1 flex flex-row justify-between w-full items-center">
                <Button variant = "outline" class = "w-full justify-between">
                    <span class = "text-lg font-semibold text-foreground">Accounts</span>
                    <Handshake class = "flex justify-end mr-0 bg-red-500" size={titlebar_icon_size} />
                </Button>
            </div>

            
        </div>


    </Sheet.Content>
</Sheet.Root> -->