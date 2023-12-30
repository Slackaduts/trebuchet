<script lang="ts">
    import { popup } from '@skeletonlabs/skeleton';
    import type { PopupSettings } from '@skeletonlabs/skeleton';
    import { fade } from 'svelte/transition';

    import Help from '$lib/icons/Help.svelte';

    const popupHover: PopupSettings = {
        event: 'hover',
        target: 'popupHover',
        placement: 'top',
    };

    export let content = '';
</script>


<button class="[&>*]:pointer-events-none" use:popup={popupHover}>
	<Help />
</button>

<div transition:fade class="card p-4 variant-filled-secondary duration-500" data-popup="popupHover">
	<p>{content}</p>
	<div class="arrow variant-filled-secondary" />
</div>