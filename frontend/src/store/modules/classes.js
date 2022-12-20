export default {
  state() {
    return {
      classes: [
        {
          id: 1,
          title: "5BHIF",
          initials: "5B",
          currLesson: "POS",
          students: [
            {
              name: "Dries Vincent",
              handle: "driesvincent",
              imageUrl:
                "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80",
            },
            {
              name: "Lindsay Walton",
              handle: "lindsaywalton",
              imageUrl:
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80",
            },
            {
              name: "Courtney Henry",
              handle: "courtneyhenry",
              imageUrl:
                "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80",
            },
            {
              name: "Tom Cook",
              handle: "tomcook",
              imageUrl:
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80",
            },
          ],
          totalMembers: 12,
          // lastUpdated: "March 17, 2020",
          pinned: true,
          bgColorClass: "bg-green-600",
        },
        {
          id: 2,
          title: "4AHIF",
          initials: "4A",
          currLesson: null,
          students: [
            {
              name: "Dries Vincent",
              handle: "driesvincent",
              imageUrl:
                "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxzZWFyY2h8MTB8fGZhY2V8ZW58MHx8MHx8&auto=format&fit=crop&w=800&q=60",
            },
            {
              name: "Lindsay Walton",
              handle: "lindsaywalton",
              imageUrl:
                "https://images.unsplash.com/photo-1545996124-0501ebae84d0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxzZWFyY2h8MjB8fGZhY2V8ZW58MHx8MHx8&auto=format&fit=crop&w=800&q=60",
            },
            {
              name: "Courtney Henry",
              handle: "courtneyhenry",
              imageUrl:
                "https://images.unsplash.com/photo-1601412436009-d964bd02edbc?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxzZWFyY2h8MTJ8fGZhY2V8ZW58MHx8MHx8&auto=format&fit=crop&w=800&q=60",
            },
          ],
          totalMembers: 3,
          // lastUpdated: "March 17, 2020",
          pinned: false,
          bgColorClass: "bg-gray-500",
        },
        {
          id: 2,
          title: "5AHIT",
          initials: "5A",
          currLesson: "RW (exam)",
          students: [
            {
              name: "Dries Vincent",
              handle: "driesvincent",
              imageUrl:
                "https://images.unsplash.com/photo-1544348817-5f2cf14b88c8?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxzZWFyY2h8MjN8fGZhY2V8ZW58MHx8MHx8&auto=format&fit=crop&w=800&q=60",
            },
            {
              name: "Lindsay Walton",
              handle: "lindsaywalton",
              imageUrl:
                "https://images.unsplash.com/photo-1570295999919-56ceb5ecca61?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxzZWFyY2h8Mjh8fGZhY2V8ZW58MHx8MHx8&auto=format&fit=crop&w=800&q=60",
            },
            {
              name: "Courtney Henry",
              handle: "courtneyhenry",
              imageUrl:
                "https://images.unsplash.com/photo-1534528741775-53994a69daeb?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxzZWFyY2h8Mjl8fGZhY2V8ZW58MHx8MHx8&auto=format&fit=crop&w=800&q=60",
            },
            {
              name: "Courtney Henry",
              handle: "courtneyhenry",
              imageUrl:
                "https://images.unsplash.com/photo-1552374196-c4e7ffc6e126?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxzZWFyY2h8MzJ8fGZhY2V8ZW58MHx8MHx8&auto=format&fit=crop&w=800&q=60",
            },
          ],
          totalMembers: 22,
          // lastUpdated: "March 17, 2020",
          pinned: false,
          bgColorClass: "bg-orange-500",
        },
        // More classes...
      ],
    };
  },
  getters: {
    classes: (state) => {
      return state.classes;
    },
    classById: (_state, getters) => (classId) => {
      return getters.classes.find((cl) => cl.id === classId);
    },
  },
  mutations: {
    setClasses(state, classes) {
      state.classes = classes;
    },
  },
  actions: {},
};
