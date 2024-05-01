<script lang="ts">
	// When using the Tauri API npm package:
	import { invoke } from '@tauri-apps/api/tauri'

	import { getClient, ResponseType, Response } from '@tauri-apps/api/http';
	import { onMount } from "svelte";

	const WIZARD101_NEWS_URL: string = "https://www.wizard101.com/game/news";
	const PIRATE101_NEWS_URL: string = "https://www.pirate101.com/free_game/daily_news"

	async function test() {
		const client = await getClient();
		const response: Response<string> = await client.get(PIRATE101_NEWS_URL, {
			timeout: 3,
			responseType: ResponseType.Text,
		});

		invoke("parse_pirate_news", { response: response.data });
		
	}


	onMount(async () => {
		await test();
	});
</script>



<div class = "h-full w-full flex items-center justify-center">
</div>


