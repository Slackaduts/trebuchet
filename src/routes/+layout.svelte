<script lang="ts">
	import '../app.pcss';

	import { Separator } from '$lib/components/ui/separator';

	import Titlebar from '$lib/components/titlebar/Titlebar.svelte';
	import Footer from '$lib/components/footer/Footer.svelte';
	// import { Separator } from '$lib/components/ui/separator';
	import { Button } from "$lib/components/ui/button";
	import { Progress } from "$lib/components/ui/progress";
	import * as Tooltip from "$lib/components/ui/tooltip";

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

		<div class="flex h-full">
			<slot />
		</div>

		<div class="flex flex-col h-fit items-center w-full">
			<Separator></Separator>
			<div class = "flex w-full h-fit items-center justify-between p-1 space-x-2">
				<ToggleGroup.Root type="single" variant="outline" size= "default" class = "h-10 flex">
					<Tooltip.Root>
						<Tooltip.Trigger>
							<ToggleGroup.Item value="wizard" size="default" class = "p-2">
								<WandSparkles size={titlebar_icon_size}/>
							</ToggleGroup.Item>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Wizard101</p>
						</Tooltip.Content>
					</Tooltip.Root>

					<Tooltip.Root>
						<Tooltip.Trigger>
							<ToggleGroup.Item value="pirate" size="default" class = "p-2">
								<ShipWheel size={titlebar_icon_size}/>
							</ToggleGroup.Item>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Pirate101</p>
						</Tooltip.Content>
					</Tooltip.Root>
				</ToggleGroup.Root>
				<Progress value={33} class="h-[20%]" />
				<Button variant="outline" class = "px-5 py-4 text-2xl">Play</Button>
			</div>
			<Separator></Separator>
			<Footer></Footer>
		</div>
	</div>
</body>
