import { writable } from 'svelte/store';

export const news_store = writable([{date: "", title: "", image: "", link: "", content: ""}]);
export const news_url_store = writable("lol");
export const game_store = writable("pirate");