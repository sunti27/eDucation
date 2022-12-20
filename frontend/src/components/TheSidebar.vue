<template>
  <TransitionRoot as="template" :show="open">
    <Dialog as="div" class="relative z-40 lg:hidden" @close="close">
      <TransitionChild
        as="template"
        enter="transition-opacity ease-linear duration-300"
        enter-from="opacity-0"
        enter-to="opacity-100"
        leave="transition-opacity ease-linear duration-300"
        leave-from="opacity-100"
        leave-to="opacity-0"
      >
        <div class="fixed inset-0 bg-gray-600 bg-opacity-75" />
      </TransitionChild>

      <div class="fixed inset-0 z-40 flex">
        <TransitionChild
          as="template"
          enter="transition ease-in-out duration-300 transform"
          enter-from="-translate-x-full"
          enter-to="translate-x-0"
          leave="transition ease-in-out duration-300 transform"
          leave-from="translate-x-0"
          leave-to="-translate-x-full"
        >
          <DialogPanel
            class="
              relative
              flex
              w-full
              max-w-xs
              flex-1 flex-col
              bg-white
              pt-5
              pb-4
            "
          >
            <TransitionChild
              as="template"
              enter="ease-in-out duration-300"
              enter-from="opacity-0"
              enter-to="opacity-100"
              leave="ease-in-out duration-300"
              leave-from="opacity-100"
              leave-to="opacity-0"
            >
              <div class="absolute top-0 right-0 -mr-12 pt-2">
                <button
                  type="button"
                  class="
                    ml-1
                    flex
                    h-10
                    w-10
                    items-center
                    justify-center
                    rounded-full
                    focus:outline-none
                    focus:ring-2
                    focus:ring-inset
                    focus:ring-white
                  "
                  @click="close"
                >
                  <span class="sr-only">Close sidebar</span>
                  <XMarkIcon class="h-6 w-6 text-white" aria-hidden="true" />
                </button>
              </div>
            </TransitionChild>
            <!-- <div class="flex flex-shrink-0 items-center px-4">
                <img
                  class="h-8 w-auto"
                  src=""
                  alt="Logo"
                />
              </div> -->
            <div class="mt-5 h-0 flex-1 overflow-y-auto">
              <nav class="px-2">
                <div class="space-y-1">
                  <a
                    v-for="item in navigation"
                    :key="item.name"
                    :href="item.href"
                    :class="[
                      item.current
                        ? 'bg-gray-100 text-gray-900'
                        : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50',
                      'group flex items-center px-2 py-2 text-base leading-5 font-medium rounded-md',
                    ]"
                    :aria-current="item.current ? 'page' : undefined"
                  >
                    <component
                      :is="item.icon"
                      :class="[
                        item.current
                          ? 'text-gray-500'
                          : 'text-gray-400 group-hover:text-gray-500',
                        'mr-3 flex-shrink-0 h-6 w-6',
                      ]"
                      aria-hidden="true"
                    />
                    {{ item.name }}
                  </a>
                </div>
              </nav>
            </div>
          </DialogPanel>
        </TransitionChild>
        <div class="w-14 flex-shrink-0" aria-hidden="true">
          <!-- Dummy element to force sidebar to shrink to fit close icon -->
        </div>
      </div>
    </Dialog>
  </TransitionRoot>

  <!-- Static sidebar for desktop -->
  <div
    class="
      hidden
      z-20
      lg:fixed
      lg:inset-y-0
      lg:flex
      lg:w-64
      lg:flex-col
      lg:border-r
      lg:border-gray-200
      lg:bg-gray-100
      lg:pt-5
      lg:pb-4
    "
  >
    <!-- <div class="flex flex-shrink-0 items-center px-6">
        <img
          class="h-8 w-auto"
          src=""
          alt="Logo"
        />
      </div> -->
    <!-- Sidebar component, swap this element with another sidebar if you like -->
    <div class="mt-5 flex h-0 flex-1 flex-col overflow-y-auto pt-1">
      <!-- User account dropdown -->
      <Menu as="div" class="relative inline-block px-3 text-left">
        <div>
          <MenuButton
            class="
              group
              w-full
              rounded-md
              bg-gray-100
              px-3.5
              py-2
              text-left text-sm
              font-medium
              text-gray-700
              hover:bg-gray-200
              focus:outline-none
              focus:ring-2
              focus:ring-purple-500
              focus:ring-offset-2
              focus:ring-offset-gray-100
            "
          >
            <span class="flex w-full items-center justify-between">
              <span class="flex min-w-0 items-center justify-between space-x-3">
                <img
                  class="
                    h-10
                    w-10
                    flex-shrink-0
                    rounded-full
                    bg-gray-300
                    object-cover
                  "
                  src="https://images.unsplash.com/photo-1457449940276-e8deed18bfff?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1770&q=80"
                  alt=""
                />
                <span class="flex min-w-0 flex-1 flex-col">
                  <span class="truncate text-sm font-medium text-gray-900"
                    >Fabian Wassermann</span
                  >
                  <span class="truncate text-sm text-gray-500">@wassermf</span>
                </span>
              </span>
              <ChevronUpDownIcon
                class="
                  h-5
                  w-5
                  flex-shrink-0
                  text-gray-400
                  group-hover:text-gray-500
                "
                aria-hidden="true"
              />
            </span>
          </MenuButton>
        </div>
        <transition
          enter-active-class="transition ease-out duration-100"
          enter-from-class="transform opacity-0 scale-95"
          enter-to-class="transform opacity-100 scale-100"
          leave-active-class="transition ease-in duration-75"
          leave-from-class="transform opacity-100 scale-100"
          leave-to-class="transform opacity-0 scale-95"
        >
          <MenuItems
            class="
              absolute
              right-0
              left-0
              z-30
              mx-3
              mt-1
              origin-top
              divide-y divide-gray-200
              rounded-md
              bg-white
              shadow-lg
              ring-1 ring-black ring-opacity-5
              focus:outline-none
            "
          >
            <div class="py-1">
              <MenuItem v-slot="{ active }">
                <a
                  href="#"
                  :class="[
                    active ? 'bg-gray-100 text-gray-900' : 'text-gray-700',
                    'block px-4 py-2 text-sm',
                  ]"
                  >View profile</a
                >
              </MenuItem>
              <MenuItem v-slot="{ active }">
                <a
                  href="#"
                  :class="[
                    active ? 'bg-gray-100 text-gray-900' : 'text-gray-700',
                    'block px-4 py-2 text-sm',
                  ]"
                  >Settings</a
                >
              </MenuItem>
            </div>
            <div class="py-1">
              <MenuItem v-slot="{ active }">
                <a
                  href="#"
                  :class="[
                    active ? 'bg-gray-100 text-gray-900' : 'text-gray-700',
                    'block px-4 py-2 text-sm',
                  ]"
                  >Logout</a
                >
              </MenuItem>
            </div>
          </MenuItems>
        </transition>
      </Menu>
      <!-- Sidebar Search -->
      <div class="mt-5 px-3">
        <label for="search" class="sr-only">Search</label>
        <div class="relative mt-1 rounded-md shadow-sm">
          <div
            class="
              pointer-events-none
              absolute
              inset-y-0
              left-0
              flex
              items-center
              pl-3
              z-10
            "
            aria-hidden="true"
          >
            <MagnifyingGlassIcon
              class="mr-3 h-4 w-4 text-gray-400"
              aria-hidden="true"
            />
          </div>
          <input
            type="text"
            name="search"
            id="search"
            class="
              block
              w-full
              rounded-md
              border-gray-300
              pl-9
              focus:border-indigo-500 focus:ring-indigo-500
              sm:text-sm
            "
            placeholder="Search"
          />
        </div>
      </div>
      <!-- Navigation -->
      <nav class="mt-6 px-3">
        <div class="space-y-1">
          <a
            v-for="item in navigation"
            :key="item.name"
            :href="item.href"
            :class="[
              item.current
                ? 'bg-gray-200 text-gray-900'
                : 'text-gray-700 hover:text-gray-900 hover:bg-gray-50',
              'group flex items-center px-2 py-2 text-sm font-medium rounded-md',
            ]"
            :aria-current="item.current ? 'page' : undefined"
          >
            <component
              :is="item.icon"
              :class="[
                item.current
                  ? 'text-gray-500'
                  : 'text-gray-400 group-hover:text-gray-500',
                'mr-3 flex-shrink-0 h-6 w-6',
              ]"
              aria-hidden="true"
            />
            {{ item.name }}
          </a>
        </div>
      </nav>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import {
  Dialog,
  DialogPanel,
  Menu,
  MenuButton,
  MenuItem,
  MenuItems,
  TransitionChild,
  TransitionRoot,
} from "@headlessui/vue";
import {
  Bars3CenterLeftIcon,
  Bars4Icon,
  ClockIcon,
  HomeIcon,
  XMarkIcon,
} from "@heroicons/vue/24/outline";
import {
  ChevronRightIcon,
  ChevronUpDownIcon,
  EllipsisVerticalIcon,
  MagnifyingGlassIcon,
} from "@heroicons/vue/20/solid";

const props = defineProps({
  open: Boolean,
});

const emit = defineEmits(["close"]);

const close = () => {
  emit("close");
};

const navigation = [
  { name: "Home", href: "#", icon: HomeIcon, current: true },
  // { name: "My tasks", href: "#", icon: Bars4Icon, current: false },
  // { name: "Recent", href: "#", icon: ClockIcon, current: false },
];
// const teams = [
//   { name: "Engineering", href: "#", bgColorClass: "bg-indigo-500" },
//   { name: "Human Resources", href: "#", bgColorClass: "bg-green-500" },
//   { name: "Customer Success", href: "#", bgColorClass: "bg-yellow-500" },
// ];
</script>