<template>
  <div class="min-h-full">
    <TheSidebar :open="sidebarOpen" @close="sidebarOpen = false"></TheSidebar>
    <!-- Main column -->
    <div class="flex flex-col lg:pl-64 z-10">
      <!-- Search header -->
      <div
        class="
          sticky
          top-0
          z-10
          flex
          h-16
          flex-shrink-0
          border-b border-gray-200
          bg-white
          lg:hidden
        "
      >
        <button
          type="button"
          class="
            border-r border-gray-200
            px-4
            text-gray-500
            focus:outline-none
            focus:ring-2
            focus:ring-inset
            focus:ring-purple-500
            lg:hidden
          "
          @click="sidebarOpen = true"
        >
          <span class="sr-only">Open sidebar</span>
          <Bars3CenterLeftIcon class="h-6 w-6" aria-hidden="true" />
        </button>
        <div class="flex flex-1 justify-between px-4 sm:px-6 lg:px-8">
          <div class="flex flex-1">
            <form class="flex w-full md:ml-0" action="#" method="GET">
              <label for="search-field" class="sr-only">Search</label>
              <div
                class="relative w-full text-gray-400 focus-within:text-gray-600"
              >
                <div
                  class="
                    pointer-events-none
                    absolute
                    inset-y-0
                    left-0
                    flex
                    items-center
                    z-10
                  "
                >
                  <MagnifyingGlassIcon class="h-5 w-5" aria-hidden="true" />
                </div>
                <input
                  id="search-field"
                  name="search-field"
                  class="
                    block
                    h-full
                    w-full
                    border-transparent
                    py-2
                    pl-8
                    pr-3
                    text-gray-900
                    placeholder-gray-500
                    focus:border-transparent
                    focus:placeholder-gray-400
                    focus:outline-none
                    focus:ring-0
                    sm:text-sm
                  "
                  placeholder="Search"
                  type="search"
                />
              </div>
            </form>
          </div>
          <div class="flex items-center">
            <!-- Profile dropdown -->
            <Menu as="div" class="relative ml-3">
              <div>
                <MenuButton
                  class="
                    flex
                    max-w-xs
                    items-center
                    rounded-full
                    bg-white
                    text-sm
                    focus:outline-none
                    focus:ring-2
                    focus:ring-purple-500
                    focus:ring-offset-2
                  "
                >
                  <span class="sr-only">Open user menu</span>
                  <img
                    class="h-8 w-8 rounded-full object-cover"
                    src="https://images.unsplash.com/photo-1457449940276-e8deed18bfff?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1770&q=80"
                    alt=""
                  />
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
                    z-10
                    mt-2
                    w-48
                    origin-top-right
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
                          active
                            ? 'bg-gray-100 text-gray-900'
                            : 'text-gray-700',
                          'block px-4 py-2 text-sm',
                        ]"
                        >View profile</a
                      >
                    </MenuItem>
                    <MenuItem v-slot="{ active }">
                      <a
                        href="#"
                        :class="[
                          active
                            ? 'bg-gray-100 text-gray-900'
                            : 'text-gray-700',
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
                          active
                            ? 'bg-gray-100 text-gray-900'
                            : 'text-gray-700',
                          'block px-4 py-2 text-sm',
                        ]"
                        >Logout</a
                      >
                    </MenuItem>
                  </div>
                </MenuItems>
              </transition>
            </Menu>
          </div>
        </div>
      </div>
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
          <div class="min-w-0 flex-1">
            <h1 class="text-lg font-medium leading-6 text-gray-900 sm:truncate">
              HTL Villach
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
        <!-- Pinned classes -->
        <div class="mt-6 px-4 sm:px-6 lg:px-8">
          <h2 class="text-sm font-medium text-gray-900">Pinned Classes</h2>
          <ul
            role="list"
            class="
              mt-3
              grid grid-cols-1
              gap-4
              sm:grid-cols-2 sm:gap-6
              xl:grid-cols-4
            "
          >
            <li
              v-for="cl in pinnedClasses"
              :key="cl.id"
              class="relative col-span-1 flex rounded-md shadow-sm"
            >
              <div
                :class="[
                  cl.bgColorClass,
                  'flex-shrink-0 flex items-center justify-center w-16 text-white text-sm font-medium rounded-l-md',
                ]"
              >
                {{ cl.initials }}
              </div>
              <div
                class="
                  flex flex-1
                  items-center
                  justify-between
                  text-ellipsis
                  whitespace-nowrap
                  rounded-r-md
                  border-t border-r border-b border-gray-200
                  bg-white
                "
              >
                <div class="flex-1 truncate px-4 py-2 text-sm">
                  <a
                    href="#"
                    class="font-medium text-gray-900 hover:text-gray-600"
                    >{{ cl.title }}</a
                  >
                  <p class="text-gray-500">{{ cl.totalMembers }} Students</p>
                </div>
                <Menu as="div" class="flex-shrink-0 pr-2">
                  <MenuButton
                    class="
                      inline-flex
                      h-8
                      w-8
                      items-center
                      justify-center
                      rounded-full
                      bg-white
                      text-gray-400
                      hover:text-gray-500
                      focus:outline-none
                      focus:ring-2
                      focus:ring-purple-500
                      focus:ring-offset-2
                    "
                  >
                    <span class="sr-only">Open options</span>
                    <EllipsisVerticalIcon class="h-5 w-5" aria-hidden="true" />
                  </MenuButton>
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
                        right-10
                        top-3
                        z-20
                        mx-3
                        mt-1
                        w-48
                        origin-top-right
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
                              active
                                ? 'bg-gray-100 text-gray-900'
                                : 'text-gray-700',
                              'block px-4 py-2 text-sm',
                            ]"
                            >Join</a
                          >
                        </MenuItem>
                      </div>
                      <div class="py-1">
                        <MenuItem v-slot="{ active }">
                          <a
                            href="#"
                            :class="[
                              active
                                ? 'bg-gray-100 text-gray-900'
                                : 'text-gray-700',
                              'block px-4 py-2 text-sm',
                            ]"
                            >Removed from pinned</a
                          >
                        </MenuItem>
                      </div>
                    </MenuItems>
                  </transition>
                </Menu>
              </div>
            </li>
          </ul>
        </div>

        <!-- Projects list (only on smallest breakpoint) -->
        <div class="mt-10 sm:hidden">
          <div class="px-4 sm:px-6">
            <h2 class="text-sm font-medium text-gray-900">Classrooms</h2>
          </div>
          <ul
            role="list"
            class="mt-3 divide-y divide-gray-100 border-t border-gray-200"
          >
            <li v-for="cl in classes" :key="cl.id">
              <a
                href="#"
                class="
                  group
                  flex
                  items-center
                  justify-between
                  px-4
                  py-4
                  hover:bg-gray-50
                  sm:px-6
                "
              >
                <span class="flex items-center space-x-3 truncate">
                  <span
                    :class="[
                      cl.bgColorClass,
                      'w-2.5 h-2.5 flex-shrink-0 rounded-full',
                    ]"
                    aria-hidden="true"
                  />
                  <span class="truncate text-sm font-medium leading-6">
                    {{ cl.title }}
                    {{ " " }}
                    <span class="truncate font-normal text-gray-500"
                      >in {{ cl.currLesson }}</span
                    >
                  </span>
                </span>
                <ChevronRightIcon
                  class="ml-4 h-5 w-5 text-gray-400 group-hover:text-gray-500"
                  aria-hidden="true"
                />
              </a>
            </li>
          </ul>
        </div>

        <!-- Projects table (small breakpoint and up) -->
        <div class="mt-8 hidden sm:block">
          <div
            class="
              inline-block
              min-w-full
              border-b border-gray-200
              align-middle
            "
          >
            <table class="min-w-full">
              <thead>
                <tr class="border-t border-gray-200">
                  <th
                    class="
                      border-b border-gray-200
                      bg-gray-50
                      px-6
                      py-3
                      text-left text-sm
                      font-semibold
                      text-gray-900
                    "
                    scope="col"
                  >
                    <span class="lg:pl-2">Classes</span>
                  </th>
                  <th
                    class="
                      border-b border-gray-200
                      bg-gray-50
                      px-6
                      py-3
                      text-left text-sm
                      font-semibold
                      text-gray-900
                    "
                    scope="col"
                  >
                    Students
                  </th>
                  <!-- <th
                    class="
                      hidden
                      border-b border-gray-200
                      bg-gray-50
                      px-6
                      py-3
                      text-right text-sm
                      font-semibold
                      text-gray-900
                      md:table-cell
                    "
                    scope="col"
                  >
                    Row
                  </th> -->
                  <th
                    class="
                      border-b border-gray-200
                      bg-gray-50
                      py-3
                      pr-6
                      text-right text-sm
                      font-semibold
                      text-gray-900
                    "
                    scope="col"
                  />
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-100 bg-white">
                <tr v-for="cl in classes" :key="cl.id">
                  <td
                    class="
                      w-full
                      max-w-0
                      whitespace-nowrap
                      px-6
                      py-3
                      text-sm
                      font-medium
                      text-gray-900
                    "
                  >
                    <div class="flex items-center space-x-3 lg:pl-2">
                      <div
                        :class="[
                          cl.bgColorClass,
                          'flex-shrink-0 w-2.5 h-2.5 rounded-full',
                        ]"
                        aria-hidden="true"
                      />
                      <a href="#" class="truncate hover:text-gray-600">
                        <span>
                          {{ cl.title }}
                          {{ " " }}
                          <span
                            v-if="cl.currLesson"
                            class="font-normal text-gray-500"
                            >in {{ cl.currLesson }}</span
                          >
                          <span v-else class="font-normal text-gray-500"
                            >no lesson</span
                          >
                        </span>
                      </a>
                    </div>
                  </td>
                  <td class="px-6 py-3 text-sm font-medium text-gray-500">
                    <div class="flex items-center space-x-2">
                      <div class="flex flex-shrink-0 -space-x-1">
                        <img
                          v-for="member in cl.students"
                          :key="member.handle"
                          class="
                            h-6
                            w-6
                            max-w-none
                            rounded-full
                            ring-2 ring-white
                            object-cover
                          "
                          :src="member.imageUrl"
                          :alt="member.name"
                        />
                      </div>
                      <span
                        v-if="cl.totalMembers > cl.students.length"
                        class="flex-shrink-0 text-xs font-medium leading-5"
                        >+{{ cl.totalMembers - cl.students.length }}</span
                      >
                    </div>
                  </td>
                  <!-- <td
                    class="
                      hidden
                      whitespace-nowrap
                      px-6
                      py-3
                      text-right text-sm text-gray-500
                      md:table-cell
                    "
                  >
                    {{ cl.lastUpdated }}
                  </td> -->
                  <td
                    class="
                      whitespace-nowrap
                      px-6
                      py-3
                      text-right text-sm
                      font-medium
                    "
                  >
                    <a href="#" class="text-indigo-600 hover:text-indigo-900"
                      >Join</a
                    >
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>
  
<script setup>
import { ref, computed } from "vue";
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
import TheSidebar from "@/components/TheSidebar.vue";
import { useStore } from "vuex";
const store = useStore();

const classes = computed(() => store.getters.classes);
const pinnedClasses = computed(() => classes.value.filter((cl) => cl.pinned));

const sidebarOpen = ref(false);
</script>