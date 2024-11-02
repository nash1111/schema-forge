<!-- src/pages/OpenAPIUploader.vue -->
<template>
    <div class="drop-zone" @drop="handleDrop" @dragover.prevent>
        <p>upload OpenAPI file</p>
    </div>
    <button @click="saveOpenAPI">保存</button>
    <pre>{{ openApiData }}</pre>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";

const openApiData = ref(null);

async function handleDrop(event: DragEvent) {
    event.preventDefault();
    const files = event.dataTransfer?.files;

    if (files && files.length > 0) {
        const file = files[0];

        if (file.type === "application/x-yaml" || file.name.endsWith(".yml") || file.name.endsWith(".yaml")) {
            try {
                openApiData.value = await invoke("load_openapi", { filePath: file.path });
            } catch (error) {
                console.error("Failed to load OpenAPI data:", error);
            }
        } else {
            console.warn("YAMLファイルをドロップしてください");
        }
    }
}

async function saveOpenAPI() {
    try {
        await invoke("save_openapi", { data: openApiData.value, filePath: "path/to/save.yml" });
        console.log("YAMLファイルが保存されました");
    } catch (error) {
        console.error("Failed to save OpenAPI data:", error);
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
}

.drop-zone:hover {
    border-color: #333;
}
</style>