<script lang="ts">
    export let lemma: string;
    export let submit: VoidFunction;
    export let toggle: VoidFunction = () => {};
</script>

<form class="join">
    <input
        type="text"
        bind:value={lemma}
        class="input input-bordered w-full join-item"
        required
    />
    <button class="btn-primary join-item" on:click={submit} on:click={toggle}>
        Save
    </button>
</form>
