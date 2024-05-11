<script lang="ts">
    // import * as Sheet from "$lib/components/ui/sheet";
    import * as HoverCard from "$lib/components/ui/hover-card";
    import * as ToggleGroup from "$lib/components/ui/toggle-group";
    import Separator from "$lib/components/ui/separator/separator.svelte";

    import {Bolt, Handshake, Newspaper, Terminal} from 'lucide-svelte';

    export let titlebar_icon_size = 24;
</script>
 

<div data-tauri-drag-region class = "flex flex-row w-full justify-start items-center space-x-1">
    <ToggleGroup.Root type="single" variant="ghost" size= "default" class = "h-10 flex">
        <a href = "/settings">
            <HoverCard.Root>
                <HoverCard.Trigger>
                    <ToggleGroup.Item value="settings" size="default" class = "p-2" >
                        <Bolt size={titlebar_icon_size}/>
                    </ToggleGroup.Item>
                </HoverCard.Trigger>
                <HoverCard.Content>
                    <h1 class = "text-lg font-semibold text-foreground">Settings</h1>
                    <Separator/>
                    Configure Trebuchet in a few easy clicks.
                </HoverCard.Content>
            </HoverCard.Root>
        </a>

        <a href = "/accounts">
            <HoverCard.Root>
                <HoverCard.Trigger>
                    <ToggleGroup.Item value="accounts" size="default" class = "p-2"> 
                        <Handshake size={titlebar_icon_size}/>
                    </ToggleGroup.Item>
                </HoverCard.Trigger>
                <HoverCard.Content>
                    <h1 class = "text-lg font-semibold text-foreground">Accounts</h1>
                    <Separator/>
                    Add, remove, and managed your Wizard101/Pirate101 accounts.
                </HoverCard.Content>
            </HoverCard.Root>
        </a>

        <a href = "/">
            <HoverCard.Root>
                <HoverCard.Trigger>
                    <ToggleGroup.Item value="home" size="default" class = "p-2">
                        <Newspaper size={titlebar_icon_size}/>
                    </ToggleGroup.Item>
                </HoverCard.Trigger>
                <HoverCard.Content>
                    <h1 class = "text-lg font-semibold text-foreground">News</h1>
                    <Separator/>
                    See news related to Pirate101/Wizard101.
                </HoverCard.Content>
            </HoverCard.Root>
        </a>

        <a href = "/logs">
            <HoverCard.Root>
                <HoverCard.Trigger>
                    <ToggleGroup.Item value="logs" size="default" class = "p-2">
                        <Terminal size={titlebar_icon_size}/>
                    </ToggleGroup.Item>
                </HoverCard.Trigger>
                <HoverCard.Content>
                    <h1 class = "text-lg font-semibold text-foreground">Logs</h1>
                    <Separator/>
                    See live debug information for Trebuchet.
                </HoverCard.Content>
            </HoverCard.Root>
        </a>
    </ToggleGroup.Root>
</div>