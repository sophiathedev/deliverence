<script setup lang="ts">
import {get, has, set} from 'tauri-plugin-cache-api';
import {message, open} from '@tauri-apps/plugin-dialog';
import {onMounted, ref} from "vue";
import Button from "./components/Button.vue";
import {invoke} from "@tauri-apps/api/core";

type SenderData = { name: string, email: string, password: string }

const existCachedSenderData = ref(false);
const cachedSenderData = ref<SenderData[] | null>([]);

const cachedReceiverData: any = ref({});
const isLoadedReceiverData: any = ref({})
const receiverField: any = ref([])
const templatePath: any = ref({});
const emailSubject: any = ref({} as Record<string, string>);
onMounted(async () => {
  existCachedSenderData.value = await has('cached_sender_data_list');
  if (existCachedSenderData.value) {
    cachedSenderData.value = await get<[SenderData] | null>('cached_sender_data_list');

    console.log(cachedSenderData.value);
  }

  for (let k of cachedSenderData.value!) {
    // initialize a subject entry keyed by sender email so inputs can bind via v-model
    emailSubject.value[k.email] = '';
    // preserve existing initialization for receiver loaded flag
    isLoadedReceiverData.value[k.email] = false;
  }

})


async function openReceiverListFileDialog(email: string) {
  const filePath = await open({
    multiple: false,
    directory: false,
  })


  let data: [Map<string, string>] = await invoke("read_receiver_csv_list", {filePath: filePath});

  isLoadedReceiverData.value[email] = true;
  cachedReceiverData.value[email] = data;
  receiverField.value = Object.keys(cachedReceiverData.value[email][0]);

  await set('cached_receiver:' + email, data);
}

async function openTemplateDialog(email: string) {
  const path = await open({
    multiple: false,
    directory: false,
    filters: [
      {name: 'Template files', extensions: ['html', 'htm', 'txt']}
    ]
  })

  templatePath.value[email] = path;
}

const isExecuting = ref(false);
const timeout = ref(20);

async function execute() {
  isExecuting.value = true;

  await invoke("send_email_by_thread", {
    senderData: cachedSenderData.value!,
    receiverData: cachedReceiverData.value!,
    templatePath: templatePath.value,
    timeout: timeout.value,
    emailSubject: emailSubject.value,
  }).then(async () => {
    await message('All emails have been sent successfully.', {title: 'Message', kind: 'info'});
    isExecuting.value = false;
  });
}

</script>

<template>
  <div v-if="!existCachedSenderData"
       class="min-h-screen flex flex-col items-center justify-center bg-gray-100 text-gray-800">
    <div class="text-center p-8">
      <h1 class="text-6xl font-extrabold text-neutral-700">Oops</h1>
      <h2 class="mt-4 text-2xl text-neutral-300">No sender data found !</h2>
    </div>
  </div>
  <div v-else class="min-h-screen w-full p-3 h-screen">
    <div class="tabs tabs-xs tabs-border">
      <template v-for="(item, index) in cachedSenderData" :key="item.email">
        <input type="radio" name="my_tabs_3" class="tab border-sky-400 w-56" :aria-label="item.name ?? item.email"
               :checked="index === 0"/>
        <div class="tab-content rounded-2xl bg-base-100 border-base-300 p-3 space-y-2 h-132">
          <div class="join w-full">
            <div
                class="join-item rounded-l-lg btn btn-sm bg-neutral-300 text-white w-28 focus:outline-none text-xs btn-disabled">
              Password
            </div>
            <input :value="item.password"
                   class="input input-sm join-item rounded-r-lg outline-none w-full font-mono border-none bg-base-300/70 text-base-600"
                   disabled/>
          </div>

          <div class="flex space-x-2">
            <div class="join w-full">
              <div
                  class="join-item rounded-l-lg btn btn-sm bg-info text-white w-28 focus:outline-none text-xs btn-disabled">
                Name
              </div>
              <input v-model="item.name"
                     class="placeholder-neutral-400 input input-sm join-item rounded-r-lg focus:shadow-none outline-none w-full border-none bg-base-300/70 text-base-600"
                     placeholder="Name of sender email will display in the email announcement title." type="text"/>
            </div>

              <div class="w-full join">
              <div
                  class="join-item rounded-l-lg btn btn-sm bg-info text-white focus:outline-none text-xs btn-disabled w-28">
                Subject
              </div>
              <input v-model="emailSubject[item.email]"
                  class="placeholder-neutral-400 input input-sm join-item rounded-r-lg w-full focus:shadow-none outline-none border-none bg-base-300/70 text-base-600"
                  placeholder="Name of your email." type="text"/>
            </div>
          </div>

          <div class="join w-full">
            <div
                class="join-item rounded-l-lg btn btn-sm bg-info text-white w-28 focus:outline-none text-xs btn-disabled">
              Template
            </div>
            <input v-on:click="openTemplateDialog(item.email)" :value="templatePath[item.email]"
                   class="placeholder-neutral-400 input input-sm join-item rounded-r-lg focus:shadow-none outline-none w-full border-none bg-base-300/70 text-base-600"
                   placeholder="Path of your template." type="text"/>
          </div>

          <div class="divider !my-0.5"></div>

          <div class="pt-1.5">
            <div v-if="!isLoadedReceiverData[item.email]" class="flex flex-col justify-center items-center">
              <p class="text-sm font-normal text-neutral-400">There are no email will receive from this email.</p>
              <Button add-class="mt-2 !text-xs" v-on:click="openReceiverListFileDialog(item.email)">Load receiver
                list
              </Button>
            </div>

            <div v-else class="rounded-2xl overflow-auto h-89">
              <table class="table table-sm table-auto bg-base-200 rounded-2xl table-pin-rows">
                <thead>
                <tr class="bg-base-200 rounded-t-2xl">
                  <th class="text-center text-xs">No</th>
                  <template v-for="(k, _) in receiverField" :key="k">
                    <th class="uppercase font-normal text-xs">{{ k }}</th>
                  </template>
                </tr>
                </thead>
                <tbody>
                <template v-for="(rec, i) in cachedReceiverData[item.email]" :key="rec.email + ' ' + i">
                  <tr class="hover:bg-base-300">
                    <th class="text-center">{{ i + 1 }}</th>
                    <template v-for="(k, _) in receiverField" :key="k">
                      <th class="font-normal">{{ rec[k] }}</th>
                    </template>
                  </tr>
                </template>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </template>
    </div>

    <div class="flex justify-between items-center pt-2 space-x-2">
      <div class="space-x-2 text-xs w-40 flex items-center">
        <label>Timeout(s)</label>
        <input type="number" class="input input-sm rounded-xl focus:outline-none" placeholder="Timeout" min="0"
               v-model="timeout"
               value="20"/>
      </div>
      <div>
        <Button v-on:click="execute" add-class="!text-xs" styling="dash" variant="success" :disabled="isExecuting">
          <svg v-if="!isExecuting" xmlns="http://www.w3.org/2000/svg" fill="#ffffff" viewBox="0 0 24 24"
               stroke-width="1.5"
               stroke="currentColor" class="size-4">
            <path stroke-linecap="round" stroke-linejoin="round"
                  d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z"/>
          </svg>
          <span v-else class="loading loading-spinner loading-xs"></span>
          Execute
        </Button>
      </div>
    </div>
  </div>
</template>

<style>
@import "tailwindcss";

@plugin "daisyui";
</style>