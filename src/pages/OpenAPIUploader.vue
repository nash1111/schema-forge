<template>
    <div class="drop-zone" @drop="handleDrop" @dragover.prevent @click="triggerFileInput">
        <p>upload OpenAPI file</p>
        <input type="file" ref="fileInput" @change="handleFileSelect" style="display: none;" />
    </div>
    <button @click="loadOpenAPI">load OpenAPI</button>
    <pre>{{ openApiData }}</pre>
    <pre>{{ fileName }}</pre>
    <pre>{{ parseResult }}</pre>
    <router-link to="/">Home</router-link> 
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";

const openApiData = ref('');
const fileName = ref('');
const parseResult = ref('');
const fileInput = ref<HTMLInputElement | null>(null);

function triggerFileInput() {
    fileInput.value?.click();
}

async function handleFileSelect(event: Event) {
    const input = event.target as HTMLInputElement;
    const files = input.files;

    if (files && files.length > 0) {
        processFile(files[0]);
    }
}

async function handleDrop(event: DragEvent) {
    event.preventDefault();
    const files = event.dataTransfer?.files;

    if (files && files.length > 0) {
        processFile(files[0]);
    }
}

async function processFile(file: File) {
    fileName.value = file.name;

    if (file.type === "application/x-yaml" || file.name.endsWith(".yml") || file.name.endsWith(".yaml")) {
        try {
            const reader = new FileReader();
            reader.onload = async (e) => {
                const content = e.target?.result as string;
                openApiData.value = content;
            };
            reader.readAsText(file);
        } catch (error) {
            console.error("Failed to load OpenAPI data:", error);
        }
    } else {
        console.warn("upload OpenAPI file");
    }
}

async function loadOpenAPI() {
    if (!openApiData.value) {
        console.warn("OpenAPI data is empty");
        return;
    }

    try {
        parseResult.value = await invoke("parse_openapi", { content: openApiData.value });
    } catch (error) {
        parseResult.value = `Error: ${error}`;
        console.error("Failed to load OpenAPI data:", error);
    }
}

</script>

<style scoped>
.drop-zone {
    width: 100%;
    height: 200px;
    border: 2px dashed #aaa;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    color: #555;
    cursor: pointer;
}

.drop-zone:hover {
    border-color: #333;
}
</style>