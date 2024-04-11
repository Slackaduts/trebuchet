<script lang="ts">
	import * as Carousel from "$lib/components/ui/carousel/index.js";

	// When using the Tauri API npm package:
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/shell';
	import { onMount } from "svelte";

	let image_urls: string[] = [];
	let link_urls: string[] = [];
	onMount(async () => {
		[image_urls, link_urls] = await invoke('get_slideshow_images');
  	});
</script>



<div class = "h-full w-full flex items-center justify-center">
	<Carousel.Root  opts={{ align: "center", loop: true, }} class = " min-h-sm max-h-sm min-w-sm max-w-sm items-center bg-red-400">
		<Carousel.Content class = "max-h-sm max-w-sm items-center w-fit">
			{#each image_urls as url, i}
				<Carousel.Item class = "flex justify-center items-center">
					<button on:click = {() => open(link_urls[i])}>
						<img title="" alt="" src={url} width={(293 * 2)} height={(320 * 2)}>
					</button>
				</Carousel.Item>
			{/each}
		</Carousel.Content>
		<Carousel.Previous />
		<Carousel.Next />
	</Carousel.Root>
</div>