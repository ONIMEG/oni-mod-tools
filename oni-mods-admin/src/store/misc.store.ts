import { defineStore } from 'pinia';

export const useMiscValuesStore = defineStore({
  id: 'misc-values',
  state: () => ({
    value2: 0,
    value3: false,
  }),
});
