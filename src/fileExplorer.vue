<template>
	<div>
		<button @click="browseFolder">选择文件夹</button>
		<FileTree v-if="root" :files="[root]" />
	</div>
</template>
  
<script>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api';

import FileTree from './fileTree.vue';

export default {
	name: 'FileExplorer',
	components: {
		FileTree
	},
	setup() {
		const root = ref(null);

		const browseFolder = async () => {
			const result = await invoke('open_dialog', {
				directory: true
			});

			if (result && result.length > 0) {
				const folderPath = result[0];
				root.value = {
					path: folderPath,
					name: getFolderName(folderPath),
					collapsed: false,
					children: getFiles(folderPath)
				};
			}
		};

		const getFolderName = (folderPath) => {
			const pathParts = folderPath.split('/');
			return pathParts[pathParts.length - 1];
		};

		const getFiles = (folderPath) => {
			const fs = require('fs');
			const path = require('path');

			const files = [];
			const entries = fs.readdirSync(folderPath, { withFileTypes: true });

			entries.forEach((entry) => {
				const fullPath = path.join(folderPath, entry.name);
				const file = {
					path: fullPath,
					name: entry.name,
					collapsed: true
				};

				if (entry.isDirectory()) {
					file.children = getFiles(fullPath);
				}

				files.push(file);
			});

			return files;
		};

		return {
			root,
			browseFolder
		};
	}
};
</script>
  