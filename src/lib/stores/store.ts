import { writable, get, type Writable } from 'svelte/store';
// import { appLocalDataDir, BaseDirectory, join } from '@tauri-apps/api/path';
// import { createDir, writeTextFile, exists } from '@tauri-apps/api/fs';
// import { Response, ResponseType, fetch } from '@tauri-apps/api/http';

// const appLocalDataDirPath = await appLocalDataDir();

// await createDir('images', { dir: BaseDirectory.AppLocalData, recursive: true });
// await createDir('accounts', { dir: BaseDirectory.AppLocalData, recursive: true });
// await createDir('settings', { dir: BaseDirectory.AppLocalData, recursive: true });

// export const store_set = async (store: Writable<any>, value: any, path: string) => {
//     if (get(store) === value) return;
//     store.set(value);

//     await writeTextFile(path, JSON.stringify(value), {dir: BaseDirectory.AppLocalData});
// }

// Create a writable news_store and set it to the value stored in the file. If the file does not exist, set it to the default value.
export const news_store = writable([{date: "", title: "", image: "", link: "", content: ""}]);

//Check if the file exists
// const news_store_path = await join(, 'news_store.json');
// const news_store_file = await exists(await join(, 'news_store.json'), {dir: BaseDirectory.AppLocalData});





export const news_url_store = writable("lol");
export const game_store = writable("pirate");
export const images_preload_store = writable(new Map());

export const store_map_set = async (store: Writable<Map<string, any>>, key: string, value: any) => {
    const s = get(store);
    s.set(key, value);
    store.set(s);
}