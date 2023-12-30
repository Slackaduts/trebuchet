<script lang="ts">
    import { SlideToggle, FileButton } from '@skeletonlabs/skeleton';
    import Tooltip from '$lib/components/Tooltip.svelte';
    import Folder from '$lib/icons/Folder.svelte';

    let skipMissing = false;
    let skipPatching = false;
    let verboseLogs = false;

    let wizard101Paths: FileList;
    let pirate101Paths: FileList;
</script>




<div class = "w-full h-full bg-purple-900">
    <div class = "grid grid-cols-3 grid-rows-3 gap-4 items-center">
        <div class = "items-center flex flex-row gap-2">
            <p>Skip Missing Files</p>
            <SlideToggle name="slide1" bind:checked={skipMissing} />
            <Tooltip content = "Skip missing files in the game's install directory. Useful for if you don't want to download the whole game."/>
        </div>
        
        <div class = "items-center flex flex-row gap-2">
            <p>Skip Patching</p>
            <SlideToggle name="slide2" bind:checked={skipPatching} />
            <Tooltip content = "Whether or not patching the game should be skipped before launching."/>
        </div>

        <div class = "items-center flex flex-row gap-2">
            <p>Verbose Logging</p>
            <SlideToggle name="slide3" bind:checked={verboseLogs} />
            <Tooltip content = "Whether or not logs should be fully detailed and contain debug information."/>
        </div>

        <div class = "flex flex-row items-center col-span-3 bg-red-300">
            {#if wizard101Paths && wizard101Paths.length > 0}
                <p>{wizard101Paths.item(0)?.name}</p>
            {/if}

            <FileButton accept="webkitdirectory" name = "wizard101path" bind:files={wizard101Paths} class = "flex bg-blue-400 w-full">
                <div class = "flex flex-row items-center justify-center m-auto w-full bg-green-600">
                    <p>Wizard101 Path</p>
                    <Folder />
                </div>
            </FileButton>
        </div>

        <div class = "flex flex-row items-center col-span-3 bg-red-300 justify-between">
            {#if pirate101Paths && pirate101Paths.length > 0}
                <p>{pirate101Paths.item(0)?.name}</p>
            {/if}

            <FileButton accept="webkitdirectory" name = "pirate101101path" bind:files={pirate101Paths} class = "flex bg-blue-400 w-full">
                <div class = "flex flex-row items-center justify-center m-auto w-full bg-green-600">
                    <p>Pirate101 Path</p>
                    <Folder />
                </div>
            </FileButton>
        </div>
    </div>
</div>






