import { type Writable, writable } from "svelte/store";

const GolonganKataStore: Writable<String[]> = writable([]);

export default GolonganKataStore;
