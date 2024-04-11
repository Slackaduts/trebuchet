<script lang="ts">
	import "$lib/../app.pcss";
    // import '$lib/../app.postcss';

    import Close from '$lib/components/titlebar/Close.svelte';
    import ToggleMaximize from '$lib/components/titlebar/ToggleMaximize.svelte';
    import Minimize from "$lib/components/titlebar/Minimize.svelte";

	import UpdateChecker from "$lib/components/titlebar/UpdateChecker.svelte";
    import NavigationBar from "$lib/components/titlebar/NavigationBar.svelte";

    let titlebar_icon_size = 24;
</script>


<div data-tauri-drag-region class="h-fit grid grid-cols-3 justify-center items-center p-1">
    <div data-tauri-drag-region class = "justify-start">
        <!-- <UpdateChecker titlebar_icon_size = {titlebar_icon_size}/> -->
        <NavigationBar titlebar_icon_size = {titlebar_icon_size}/>
    </div>
    <div data-tauri-drag-region class = "justify-center flex">
        <h1 class = "text-2xl font-bold">Trebuchet</h1>
    </div>
    <div data-tauri-drag-region class = "flex justify-end space-x-1">
        <Minimize size = {titlebar_icon_size}/>
        <ToggleMaximize size = {titlebar_icon_size}/>
        <Close size = {titlebar_icon_size} />
    </div>
</div>