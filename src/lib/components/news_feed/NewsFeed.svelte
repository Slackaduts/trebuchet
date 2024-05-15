<script lang="ts">
    import { get } from 'svelte/store';
    import { news_store, news_url_store, images_preload_store } from '$lib/stores/store';

    import { Response, ResponseType, fetch } from '@tauri-apps/api/http';

    import { invoke } from '@tauri-apps/api/tauri'
	import { appLocalDataDir } from '@tauri-apps/api/path';
	import { onMount } from "svelte";

    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
	import * as Accordion from "$lib/components/ui/accordion";

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

    let live_news: NewsEntry[] = [];

    const get_news = async (news_url: string, func: string) => {
        if (get(news_url_store) == news_url) { return; }

        news_url_store.set(news_url.toString());

        const response: Response<string> = await fetch(news_url, {
            method: 'GET',
            timeout: 3,
            responseType: ResponseType.Text,
        });

        let news: NewsEntry[] = [];
        await invoke(func, { response: response.data }).then((result) => {

            news = result as NewsEntry[];
        });

        news_store.set(news);
    }

    const preload_images = async (images: string[]) => {
        let news = get(news_store);
        let preloaded_images = get(images_preload_store);

        for (let i = 0; i < news.length, i++;) {
            let news_entry = news[i];
            if (preloaded_images.has(news_entry.image)) { continue; }

            // Download the image
            

        };

        images_preload_store.set(preloaded_images);
    }

	onMount(async () => {
		// const appLocalDataDirPath = await appLocalDataDir();
		// await get_news(WIZARD101_NEWS_URL, WIZARD101_NEWS_FUNC);
		await get_news(PIRATE101_NEWS_URL, PIRATE101_NEWS_FUNC);
		news_store.subscribe((value) => {
			live_news = value;
		});
	});
</script>

<div class = "h-full w-full items-center justify-center text-center">
    <!-- <h1 class="py-10 text-4xl font-bold">News</h1> -->
    <ScrollArea class="h-[380px] w-full rounded-md border p-2">
        <Accordion.Root>
            {#each live_news as item, i}
            <Accordion.Item value="item-{i}">
                <Accordion.Trigger>{item.title}</Accordion.Trigger>
                <Accordion.Content>
                    <div class = "flex flex-row items-center">
                        <!-- {#await preload(item.image) then _} -->
                        <img data-sveltekit-preload = "eager" src={item.image} alt={item.title} class="w-[100px] h-[100px]">
                        <!-- {/await} -->
                        <!-- <img src={item.image} alt={item.title} class="w-[100px] h-[100px]"/> -->
                        <div class = "w-full h-[100px] py-1">
                            <div class = "flex w-full justify-center items-center">
                                <div class="flex flex-col">
                                    <h5 class = "font-semibold italic">{item.date}</h5>
                                </div>
                            </div>
                            <div class = "p-2 justify-left">
                                {item.content}
                            </div>
                        </div>
                    </div>
                    <!-- <Separator></Separator> -->
                </Accordion.Content>
            </Accordion.Item>
            {/each}
        </Accordion.Root>
    </ScrollArea>
</div>