<template>
	<ul>
		<li v-for="file in files" :key="file.path">
			<span @click="toggleCollapse(file)">
				{{ file.name }}
			</span>
			<FileTree v-if="!file.collapsed" :files="file.children" />
		</li>
	</ul>
</template>
  
<script>
export default {
	name: 'FileTree',
	props: {
		files: Array
	},
	data() {
		return {
			collapsed: false
		}
	},
	methods: {
		toggleCollapse(file) {
			file.collapsed = !file.collapsed;
		}
	}
};
</script>
  