<script lang="ts">
	import { AppShell, AppBar, AppRail, AppRailAnchor, AppRailTile } from '@skeletonlabs/skeleton';

	import Cog from '$lib/icons/Cog.svelte';
	import Members from '$lib/icons/Members.svelte';
	import Code from '$lib/icons/Code.svelte';
	import Rocket from '$lib/icons/Rocket.svelte';
	import PackageUpdated from '$lib/icons/PackageUpdated.svelte';

	import Accounts from '$lib/components/Accounts.svelte';
	import Launcher from '$lib/components/Launcher.svelte';
	import Settings from '$lib/components/Settings.svelte';
	import Maximize from '$lib/icons/Maximize.svelte';
	import Close from '$lib/icons/Close.svelte';
	import Minimize from '$lib/icons/Minimize.svelte';
	import Minus from '$lib/icons/Minus.svelte';
	import Mountain from '$lib/icons/Mountain.svelte';

	import { appWindow } from "@tauri-apps/api/window";

	import { fade } from 'svelte/transition';
	import Home from '$lib/icons/Home.svelte';
	// import Mountain from '$lib/icons/Mountain.svelte';

	let currentTile: number = 0;
	let isMaximized = false;

	function testFunc() {
		appWindow.toggleMaximize();
		isMaximized = !isMaximized;
	}
</script>


<AppShell>
	<svelte:fragment slot="header">
		<div data-tauri-drag-region class = "h-16 w-full grid grid-cols-3 items-center place-content-between bg-surface-100-800-token">
			<div data-tauri-drag-region class = "flex w-16 h-16 items-center justify-center drop-shadow-md">
				<PackageUpdated/>
			</div>

			<div data-tauri-drag-region class = "flex flex-row justify-center items-center drop-shadow-md">
				<h1 class = "text-2xl font-bold min-w-min drop-shadow-md">Trebuchet</h1>
			</div>

			<div data-tauri-drag-region class = "flex flex-row items-center justify-end">
				<div data-tauri-drag-region class = "flex min-h-0 w-16 items-center justify-center">
					<button class = "titlebar-button shrink" on:click = {() => {appWindow.minimize()}}>
						<Minus />
					</button>
				</div>

				<div data-tauri-drag-region class = "flex min-h-0 w-8 items-center justify-center">
					<button class = "titlebar-button shrink" on:click = {testFunc}>
						{#if isMaximized}
							<Minimize />
						{:else}
							<Maximize />
						{/if}
					</button>
				</div>

				<div data-tauri-drag-region class = "flex min-h-0 w-16 items-center justify-center">
					<button class = "titlebar-button shrink" on:click = {() => {appWindow.close()}}>
						<Close />
					</button>
				</div>
			</div>
		</div>
	</svelte:fragment>

<!-- 
	<svelte:fragment slot="sidebarLeft">
		<AppRail width = "w-16">
			<AppRailTile bind:group={currentTile} name="tile-1" value={0} title="Launcher">
					<svelte:fragment slot="lead">
						<div class = "m-auto flex flex-col items-center place-content-center transition-all ease-in duration-300">
							<Home />
						</div>
					</svelte:fragment>
			</AppRailTile>

			<AppRailTile bind:group={currentTile} name="tile-2" value={1} title="Accounts">
				<svelte:fragment slot="lead">
					<div class = "m-auto flex flex-col items-center place-content-center transition-all ease-in duration-300">
						<Members />
					</div>
				</svelte:fragment>
			</AppRailTile>

			<AppRailTile bind:group={currentTile} name="tile-3" value={2} title= "Settings">
					<svelte:fragment slot="lead">
						<div class = "m-auto flex flex-col items-center justify-center transition-all ease-in duration-300">
							<Cog />
						</div>
					</svelte:fragment>
			</AppRailTile>


			<svelte:fragment slot="trail">
				<AppRailAnchor href="/" target="_blank" title="Source Code">
					<div class = "m-auto flex flex-col items-center justify-center">
						<Code />
					</div>
				</AppRailAnchor>
			</svelte:fragment>
		</AppRail>
	
	</svelte:fragment> -->




	{#if currentTile === 0}
		<div class="transition-container">
			<div transition:fade class="duration-500 absolute w-full h-full">
			<Launcher />
			</div>
		</div>
	{:else if currentTile === 1}
		<div class="transition-container">
			<div transition:fade class="duration-500 absolute w-full h-full">
			<Accounts />
			</div>
		</div>
	{:else if currentTile === 2}
		<div class="transition-container">
			<div transition:fade class="duration-500 absolute w-full h-full">
			<Settings />
			</div>
		</div>
	{/if}



	<svelte:fragment slot="pageFooter">
		<div class = "flex flex-row items-center justify-center">
			<p class = "text-sm italic">Trebuchet is a FOSS tool. If you paid for this, you were scammed.</p>
	</svelte:fragment>
</AppShell>