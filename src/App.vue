<script setup lang="ts">
import Button from "./components/Button.vue";

import {invoke} from "@tauri-apps/api/core";
import {ref} from "vue";
import {set} from "tauri-plugin-cache-api";

const btnChooseSenderData = ref(null);
const isLoading = ref(false);
const senderFileProgressBar = ref(100);
const textStatus = ref('');

async function openDialog() {
  try {
    let file_path = await invoke("open_file_dialog");
    if (file_path === "" || file_path === null) {
      isLoading.value = false;
      return;
    }

    isLoading.value = true;
    senderFileProgressBar.value = 100;

    textStatus.value = `File selected: ${file_path}`;
    
    // Wait a bit to show the status
    await new Promise(resolve => setTimeout(resolve, 100));
    
    textStatus.value = 'Parsing document...';
    let parsed_document = await invoke("read_csv", {filePath: file_path});

    if (!parsed_document || (Array.isArray(parsed_document) && parsed_document.length === 0)) {
      throw new Error('No data found in CSV file');
    }

    senderFileProgressBar.value += 400;

    textStatus.value = 'Saving data...';
    await set('cached_sender_data_list', parsed_document);
    
    // Ensure cache is written before proceeding
    await new Promise(resolve => setTimeout(resolve, 200));
    
    senderFileProgressBar.value += 400;

    textStatus.value = 'Opening sending mail window...';
    
    await new Promise(resolve => setTimeout(resolve, 100));
    
    await invoke("open_sending_mail_window");
    
    // Reset loading state after successful window open
    isLoading.value = false;
    senderFileProgressBar.value = 1000;
  } catch (error) {
    console.error('Error loading sender file:', error);
    isLoading.value = false;
    senderFileProgressBar.value = 100;
    textStatus.value = '';
    // Show error message if needed
    alert('Error loading file: ' + (error instanceof Error ? error.message : String(error)));
  }
}

async function openTemplateManagerWindow() {
  await invoke("open_template_manager_window");
}
</script>

<template>
  <div
      class="min-h-screen w-full flex flex-col items-center justify-center bg-gradient-to-br from-sky-200/70 via-white to-sky-200/70 space-y-3">
    <div>
      <p v-if="!isLoading" class="text-center text-xl font-medium">
        You must choose a list of sender emails.
      </p>
      <p v-else class="text-center text-xl font-medium">
        Waiting a minute, we'll processing your data...
      </p>
    </div>
    <div class="flex flex-col items-center justify-center">
      <div v-if="!isLoading" class="flex flex-col items-center justify-center space-y-2">
        <Button v-if="!isLoading" v-on:click="openDialog" ref="btnChooseSenderData" add-class="w-64">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
               class="size-4">
            <path stroke-linecap="round" stroke-linejoin="round"
                  d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m3.75 9v6m3-3H9m1.5-12H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z"/>
          </svg>

          Choose a sender data
        </Button>
        <Button v-if="!isLoading" v-on:click="openTemplateManagerWindow" add-class="w-64" styling="dash">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
          </svg>

          Template manager
        </Button>
      </div>
      <progress v-if="isLoading" class="progress progress-info w-60" :value="senderFileProgressBar" max="1000"></progress>
      <p v-if="isLoading" class="text-center text-xs text-gray-300 mt-4">{{ textStatus }}</p>
    </div>
  </div>
</template>

<style>
@import "tailwindcss";

@plugin "daisyui";
</style>