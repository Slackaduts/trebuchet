<script lang="ts">
	import '../app.pcss';
	// import '../app.pcss';
	// import '../app.postcss';

	import { Separator } from '$lib/components/ui/separator';

	import Titlebar from '$lib/components/titlebar/Titlebar.svelte';
	import Footer from '$lib/components/footer/Footer.svelte';
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
			<Footer></Footer>
		</div>
	</div>
</body>
