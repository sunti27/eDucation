<template>
  <main class="flex-1">
    <!-- Page title & actions -->
    <div
      class="
        border-b border-gray-200
        px-4
        py-4
        sm:flex sm:items-center sm:justify-between sm:px-6
        lg:px-8
      "
    >
      <div class="mr-4">
        <button
          type="button"
          class="
            inline-flex
            items-center
            rounded-md
            border border-transparent
            bg-purple-100
            pr-4
            pl-3
            py-2
            text-sm
            font-medium
            text-purple-700
            hover:bg-purple-200
            focus:outline-none
            focus:ring-2
            focus:ring-purple-500
            focus:ring-offset-2
          "
          @click="isLastCallExitClassModalOpen = true"
        >
          <ArrowLeftIcon class="h-5 w-5 mr-1"></ArrowLeftIcon>
          Back to Overview
        </button>
      </div>
      <div class="min-w-0 flex-1">
        <h1 class="text-lg font-medium leading-6 text-gray-900 sm:truncate">
          5BHIF
        </h1>
      </div>
      <div class="mt-4 flex sm:mt-0 sm:ml-4">
        <!-- <button
              type="button"
              class="
                order-0
                inline-flex
                items-center
                rounded-md
                border border-transparent
                bg-purple-600
                px-4
                py-2
                text-sm
                font-medium
                text-white
                shadow-sm
                hover:bg-purple-700
                focus:outline-none
                focus:ring-2
                focus:ring-purple-500
                focus:ring-offset-2
                sm:order-1 sm:ml-3
              "
            >
              Action button
            </button> -->
      </div>
    </div>
    <LastCallExitClass
      :open="isLastCallExitClassModalOpen"
      @close="closeLastCallExitClassModal"
    ></LastCallExitClass>
  </main>
</template>

<script setup>
import { ref, watch, computed, onMounted } from "vue";
import {} from "@headlessui/vue";
import {} from "@heroicons/vue/24/outline";
import { ArrowLeftIcon } from "@heroicons/vue/20/solid";
import { useStore } from "vuex";
import LastCallExitClass from "@/components/modals/LastCallExitClass.vue";
import { useRouter, useRoute } from "vue-router";
const store = useStore();
const route = useRoute();
const router = useRouter();

const classObj = computed(() =>
  store.getters.classes.find((cl) => route.query.classId == cl.id)
);

watch(
  () => route.query.classId,
  (newClassId) => {
    checkClassId(newClassId);
  }
);

onMounted(() => {
  checkClassId(route.query.classId);
});

const checkClassId = (classId) => {
  if (!store.getters.classes.find((cl) => classId == cl.id))
    router.push({ name: "classNotFound" });
};

const isLastCallExitClassModalOpen = ref(false);

const closeLastCallExitClassModal = (isAccepted) => {
  isLastCallExitClassModalOpen.value = false;
  if (isAccepted) router.push({ name: "overview" });
};
</script>