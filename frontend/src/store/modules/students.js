export default {
  state() {
    return {
      students: [],
    };
  },
  getters: {
    students: (state) => {
      return state.students;
    },
    studentById: (_state, getters) => (studentId) => {
      return getters.students.find((st) => st.id === studentId);
    },
  },
  mutations: {
    setStudents(state, students) {
      state.students = students;
    },
  },
  actions: {},
};
