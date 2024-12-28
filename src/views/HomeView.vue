<script lang="ts" setup>
import { onMounted, ref } from "vue";
import VirtualList from "@/components/virtual/VirtualList.vue";
import { EOF } from "dns";
import { EScrollBehavior } from "../types/virtual-list.types";

const ports = ref(
  Array.from({ length: 200 }, (_, index) => ({
    id: index + 1,
    port: 8000 + index,
    processName: `Process ${index + 1}`,
    pid: 1000 + index,
    state: "LISTENING",
  })),
);

function updatePorts() {
  if (Math.random() < 0.7) {
    const newPort = {
      id: ports.value.length
        ? Math.max(...ports.value.map((p) => p.id)) + 1
        : 1,
      port: 8000 + Math.floor(Math.random() * 100),
      processName: `NewProcess ${Math.floor(Math.random() * 100)}`,
      pid: 2000 + Math.floor(Math.random() * 100),
      state: "LISTENING",
    };
    ports.value.push(newPort);
    console.log("Added Port:", newPort);
  }

  if (ports.value.length > 50 && Math.random() < 0.3) {
    const removeIndex = Math.floor(Math.random() * ports.value.length);
    const removedPort = ports.value.splice(removeIndex, 1)[0];
    console.log("Removed Port:", removedPort);
  }

  ports.value = ports.value.map((port) => {
    if (Math.random() < 0.5) {
      return {
        ...port,
        state: ["LISTENING", "CLOSED", "ESTABLISHED"][
          Math.floor(Math.random() * 3)
        ],
      };
    }
    return port;
  });

  console.log("Updated Ports:", ports.value.length);
}

onMounted(() => {
  setInterval(updatePorts, 2000);
});

const searchModel = ref<number>(100);
const virtualListRef = ref();
</script>

<template>
  <input
    type="number"
    v-model="searchModel"
    label="search"
    placeholder="search"
  />
  <button @click.left="virtualListRef.goToElementWithIndex(searchModel)">
    Search
  </button>
  <button @click.left="virtualListRef.goToBottom">To Bottom</button>
  <button @click.left="virtualListRef.goToStart">To Top</button>

  <div style="height: 900px">
    <VirtualList
      :list="ports"
      ref="virtualListRef"
      :item-height="21"
      :scroll-behavior="EScrollBehavior.SMOOTH"
    >
      <template #item="{ item }">
        {{ item.id }} {{ item.pid }} {{ item.processName }} {{ item.port }}
      </template>
    </VirtualList>
  </div>
</template>
