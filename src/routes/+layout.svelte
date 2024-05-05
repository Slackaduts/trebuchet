<script lang="ts">
	import '../app.pcss';

	import { Separator } from '$lib/components/ui/separator';

	import Titlebar from '$lib/components/titlebar/Titlebar.svelte';
	import Footer from '$lib/components/footer/Footer.svelte';
	// import { Separator } from '$lib/components/ui/separator';
	import { Button } from "$lib/components/ui/button";
	import { Progress } from "$lib/components/ui/progress";

	import { ShipWheel } from 'lucide-svelte';
	import { WandSparkles } from 'lucide-svelte';
	import * as ToggleGroup from "$lib/components/ui/toggle-group";

	export let titlebar_icon_size = 24;
</script>

<body class="w-screen h-screen">
	<div class="w-full h-full flex flex-col justify-between">
		<div>
			<Titlebar></Titlebar>
			<Separator></Separator>
		</div>

		<div class="flex">
			<slot />
		</div>

		<div class="flex flex-col h-fit items-center w-full">
			<Separator></Separator>
			<div class = "flex w-full h-fit items-center justify-between p-2 space-x-2">
				<ToggleGroup.Root type="single" variant="outline">
					<ToggleGroup.Item value="wizard">
						<WandSparkles size={titlebar_icon_size} />
					</ToggleGroup.Item>
					<ToggleGroup.Item value="pirate" size="sm">
						<ShipWheel size={titlebar_icon_size} class="bg-red-500" />
					</ToggleGroup.Item>
				</ToggleGroup.Root>
				<Progress value={33} class="h-[20%]" />
				<Button variant="outline" class = "px-5 py-4 text-2xl">Play</Button>
			</div>
			<Separator></Separator>
			<Footer></Footer>
		</div>
	</div>
</body>
