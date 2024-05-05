<script lang="ts">
	// When using the Tauri API npm package:
	import { invoke } from '@tauri-apps/api/tauri'
	import { get } from 'svelte/store';
	import { news_store, news_url_store } from '$lib/stores/store';

	import { getClient, ResponseType, Response } from '@tauri-apps/api/http';
	import { onMount } from "svelte";

	import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
	import * as Accordion from "$lib/components/ui/accordion";

	// import { Separator } from '$lib/components/ui/separator';

	const WIZARD101_NEWS_URL: string = "https://www.wizard101.com/game/news/";
	const PIRATE101_NEWS_URL: string = "https://www.pirate101.com/free_game/daily_news"

	const WIZARD101_NEWS_FUNC: string = "parse_wizard_news";
	const PIRATE101_NEWS_FUNC: string = "parse_pirate_news";

	interface NewsEntry {
		date: string;
		title: string;
		image: string;
		link: string;
		content: string;
	}

	const get_news = async (news_url: string, func: string) => {
		if (get(news_url_store) == news_url) { return; }

		news_url_store.set(news_url.toString());

		const client = await getClient();
		const response: Response<string> = await client.get(news_url, {
			timeout: 3,
			responseType: ResponseType.Text,
		});

		let news: NewsEntry[] = [];
		await invoke(func, { response: response.data }).then((result) => {
			news = result as NewsEntry[];
		});

		news_store.set(news);
	}

	let live_news: any[] = [];

	onMount(async () => {
		// await get_news(WIZARD101_NEWS_URL, WIZARD101_NEWS_FUNC);
		await get_news(PIRATE101_NEWS_URL, PIRATE101_NEWS_FUNC);
		news_store.subscribe((value) => {
			live_news = value;
		});
	});
	// Call get_news such that it does not need to be called again upon a page reload.

</script>



<div class = "w-full h-full flex flex-col justify-between">
	<div class = "grid grid-cols-2 p-10">
		<div class = "items-center justify-center text-center ">
			<h1 class="py-10 text-4xl font-bold">News</h1>
			<ScrollArea class="h-[300px] w-full rounded-md border p-4">
				<Accordion.Root>
					{#each live_news as item, i}
					<Accordion.Item value="item-{i}">
						<Accordion.Trigger>{item.title}</Accordion.Trigger>
						<Accordion.Content>
							<div class="flex flex-row justify-center items-center">
								<!-- <img src={item.image} alt={item.title} class="w-auto h-auto" /> -->
								<!-- <Separator orientation="vertical" class = "m-10 h-auto"/> -->
								{item.content}
							</div>
						</Accordion.Content>
					</Accordion.Item>
					{/each}
				</Accordion.Root>
			</ScrollArea>
		</div>
		<!-- // Create a div that is vertically and horizontally centered and uses the full witdh and height of the parent div. -->
		<div class = "flex items-center justify-center h-full">
			<div class = "text-center">
				<h1 class="text-4xl font-bold">Wizard101</h1>
			</div>
		</div>
	</div>
	<!-- <div class="flex flex-col items-center justify-center w-full bg-purple-400">
		<Separator></Separator>
		<div class = "flex w-full h-fit items-center justify-between">
			<Select.Root>
				<Select.Trigger class="w-[180px]">
					<Select.Value placeholder="Theme" />
				</Select.Trigger>
				<Select.Content>
					<Select.Item value="light">Light</Select.Item>
					<Select.Item value="dark">Dark</Select.Item>
					<Select.Item value="system">System</Select.Item>
				</Select.Content>
			</Select.Root>
			<Progress value={33} />
			<Button class = "text-2xl">Play</Button>
		</div>
	</div> -->
</div>



