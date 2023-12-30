<script lang="ts">
    import { popup, ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
    import type { PopupSettings } from '@skeletonlabs/skeleton';
    import { ProgressBar } from '@skeletonlabs/skeleton';

    import Chevron from '$lib/icons/Chevron.svelte'

    let game = "wizard101";

    const popupCombobox: PopupSettings = {
        event: 'focus-click',
        target: 'popupCombobox',
        placement: 'bottom',
        closeQuery: '.listbox-item'
    };
				
</script>


<div class = "flex items-center justify-center w-full h-full p-8">
    <!-- <div class = "flex flex-auto bg-red-500 justify-center">
        <button class="btn variant-filled" use:popup={popupCombobox}>
            <span class="capitalize">{game ?? 'Trigger'}</span>
            <Chevron />
        </button>

        <div class="card shadow-xl py-2" data-popup="popupCombobox">
            <ListBox rounded="rounded-none">
                <ListBoxItem bind:group={game} name="medium" value="wizard101">Wizard101</ListBoxItem>
                <ListBoxItem bind:group={game} name="medium" value="pirate101">Pirate101</ListBoxItem>
            </ListBox>
            <div class="arrow bg-surface-100-800-token" />
        </div>
    </div> -->

    <div class = "flex items-center justify-center w-full h-full gap-8">
        <h3 class = "text-primary">
            Patching
        </h3>
        <ProgressBar value={undefined} track = "bg-surface-200-700-token"/>
        <button class="btn w-32 bg-gradient-to-br variant-gradient-primary-secondary">Play</button>
    </div>
</div>
