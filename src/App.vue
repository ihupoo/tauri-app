<script setup>
	import { tauri, dialog, window } from "@tauri-apps/api";

	async function selectDirectory() {
      const dirPath = await dialog.open({directory: true})
	  
	  console.log(dirPath);
	  processDirectory(dirPath)
    }

	function  processDirectory(directoryPath) {
		window.appWindow.emit('select_directory', directoryPath)
    }

</script>

<template>
	<div>
		<button @click="selectDirectory">点击</button>
	</div>
</template>

<style scoped>

</style>
